use std::collections::HashMap;

use crate::entity::Collateral;
use crate::fixed::facility::Service;
use crate::fixed::solarsystem::Solarsystem;
use crate::fixed::Statics;
use crate::persist::player::Player;
use crate::persist::site::Site;
use crate::ship::Ship;

use self::effect::apply_passives;

use super::instruction::Instruction;
use super::{Entity, Log};

mod effect;
mod module;
mod movement;

pub struct Output {
    pub remaining: Vec<Entity>,

    pub dead: Vec<Player>,
    pub docking: Vec<(Solarsystem, u8, Entity)>,
    pub warping_out: Vec<(Solarsystem, Site, Entity)>,

    pub log: Vec<Log>,
}

#[must_use]
pub fn advance(
    statics: &Statics,
    solarsystem: Solarsystem,
    site: Site,
    entities: &[Entity],
    instructions: &HashMap<usize, Vec<Instruction>>,
) -> Output {
    let mut entities = entities
        .iter()
        .enumerate()
        .map(|(i, o)| (i, o.clone()))
        .collect::<HashMap<_, _>>();

    let mut docking = Vec::new();
    let mut warping_out = Vec::new();
    let mut log = Vec::new();

    for (actor, instruction) in super::instruction::sort(instructions) {
        match instruction {
            Instruction::ModuleUntargeted(instruction) => {
                if let Some(entity) = entities.get_mut(&actor) {
                    module::apply_untargeted(statics, entity, instruction);
                }
            }
            Instruction::ModuleTargeted(instruction) => {
                module::apply_targeted(statics, &mut entities, actor, instruction, &mut log);
            }
            Instruction::SelfDestruct => {
                if let Some(entity) = entities.get_mut(&actor) {
                    module::self_destruct(entity);
                }
            }
            Instruction::Facility(instruction) => match instruction.service {
                Service::Dock => {
                    movement::dock(
                        solarsystem,
                        site,
                        &mut entities,
                        actor,
                        &mut docking,
                        &mut log,
                    );
                }
                Service::Jump => movement::jump(
                    solarsystem,
                    site,
                    &mut entities,
                    actor,
                    &mut warping_out,
                    &mut log,
                ),
            },
            Instruction::Warp(instruction) => {
                movement::warp_out(
                    solarsystem,
                    &mut entities,
                    actor,
                    instruction.target,
                    &mut warping_out,
                    &mut log,
                );
            }
        }
    }

    let (dead, remaining) = finishup_entities(statics, &entities, &mut log);

    // TODO: cleanup instructions. Warp for example has to stay there but there is a timer needed for that

    Output {
        remaining,

        dead,
        docking,
        warping_out,

        log,
    }
}

/// - apply passive effects
/// - ensure status is within ship layout limits
/// - cleanup dead
fn finishup_entities(
    statics: &Statics,
    entities: &HashMap<usize, Entity>,
    log: &mut Vec<Log>,
) -> (Vec<Player>, Vec<Entity>) {
    let mut remaining = Vec::new();
    let mut dead = Vec::new();
    for entity in entities.values() {
        match entity {
            Entity::Facility(_) => {
                remaining.push(entity.clone());
            }
            Entity::Lifeless(l) => {
                if !l.collateral.is_alive() {
                    log.push(Log::RapidUnscheduledDisassembly(entity.into()));
                } else if l.is_collapsed() {
                    log.push(Log::Collapse(entity.into()));
                } else {
                    remaining.push(entity.clone());
                }
            }
            Entity::Npc((faction, ship)) => {
                let collateral = apply_passives_and_limit_to_ship_maximum(statics, ship);
                if collateral.is_alive() {
                    remaining.push(Entity::Npc((
                        *faction,
                        Ship {
                            collateral,
                            ..ship.clone()
                        },
                    )));
                } else {
                    log.push(Log::RapidUnscheduledDisassembly(entity.into()));
                }
            }
            Entity::Player((player, ship)) => {
                let collateral = apply_passives_and_limit_to_ship_maximum(statics, ship);
                if collateral.is_alive() {
                    remaining.push(Entity::Player((
                        *player,
                        Ship {
                            collateral,
                            ..ship.clone()
                        },
                    )));
                } else {
                    log.push(Log::RapidUnscheduledDisassembly(entity.into()));
                    dead.push(*player);
                }
            }
        }
    }
    (dead, remaining)
}

fn apply_passives_and_limit_to_ship_maximum(statics: &Statics, ship: &Ship) -> Collateral {
    let layout = statics.ship_layouts.get(&ship.fitting.layout);
    let max = ship.fitting.maximum_collateral(statics);
    apply_passives(ship.collateral, &layout.round_effects).min(max)
}
