use std::collections::HashMap;

use crate::entity::Collateral;
use crate::fixed::module::targeted::Targeted;
use crate::fixed::round_effect::RoundEffect;
use crate::fixed::{module, Statics};
use crate::ship::{Cargo, CargoAmounts};
use crate::site::instruction::{UseModuleTargeted, UseModuleUntargeted};
use crate::site::{Actor, Entity, Log};

use super::effect::{apply_to_origin, apply_to_target};

pub fn self_destruct(entity: &mut Entity) {
    let collateral = match entity {
        Entity::Facility(_) => unreachable!("a facility cant self destruct"),
        Entity::Lifeless(info) => &mut info.collateral,
        Entity::Npc((_, ship)) | Entity::Player((_, ship)) => &mut ship.collateral,
    };
    *collateral = Collateral::DEAD;
}

pub fn apply_untargeted(statics: &Statics, entity: &mut Entity, instruction: UseModuleUntargeted) {
    let ship = match entity {
        Entity::Facility(_) | Entity::Lifeless(_) => {
            unreachable!("Only ships can use modules {:?}", entity)
        }
        Entity::Npc((_, ship)) | Entity::Player((_, ship)) => ship,
    };
    if let Some(module) = ship
        .fitting
        .slots_untargeted
        .get(instruction.module_index as usize)
        .map(|o| statics.modules_untargeted.get(o))
    {
        if let Some(result) = apply_to_origin(ship.collateral, &module.effects) {
            ship.collateral = result;
        }
    } else {
        println!(
            "WARN: untargeted module not found on ship {:?} {:?}",
            instruction, ship.fitting
        );
    }
}

#[must_use]
fn apply_targeted_to_origin<'s>(
    statics: &'s Statics,
    entity: &mut Entity,
    instruction: UseModuleTargeted,
) -> Option<(Actor, Targeted, &'s module::targeted::Details, CargoAmounts)> {
    let ship = match entity {
        Entity::Facility(_) | Entity::Lifeless(_) => {
            unreachable!("Only ships can use modules {:?}", entity)
        }
        Entity::Npc((_, ship)) | Entity::Player((_, ship)) => ship,
    };
    if let Some(targeted) = ship
        .fitting
        .slots_targeted
        .get(instruction.module_index as usize)
        .copied()
    {
        let details = statics.modules_targeted.get(&targeted);
        if let Some(result) = apply_to_origin(ship.collateral, &details.effects_origin) {
            ship.collateral = result;
            let free_cargo = ship.free_cargo(statics);
            return Some(((&*entity).into(), targeted, details, free_cargo));
        }
    } else {
        println!(
            "WARN: targeted module not found on ship {:?} {:?}",
            instruction, ship.fitting
        );
    }
    None
}

/// Returns loot
#[must_use]
fn apply_targeted_to_target(
    entity: &mut Entity,
    module: &module::targeted::Details,
    free_cargo: CargoAmounts,
) -> Cargo {
    match entity {
        Entity::Facility(_) => {
            // immune
            Cargo::default()
        }
        Entity::Lifeless(entity) => {
            entity.collateral = apply_to_target(entity.collateral, &module.effects_target);
            let ore = module
                .effects_target
                .iter()
                .find_map(|o| match o {
                    RoundEffect::Mine(amount) => Some(*amount),
                    _ => None,
                })
                .map_or(0, |mining_strength| {
                    let amount = mining_strength
                        .min(entity.remaining_ore)
                        .min(free_cargo.ore);
                    entity.remaining_ore -= amount;
                    amount
                });

            Cargo { ore }
        }
        Entity::Npc((_, ship)) | Entity::Player((_, ship)) => {
            ship.collateral = apply_to_target(ship.collateral, &module.effects_target);
            Cargo::default()
        }
    }
}

pub fn apply_targeted(
    statics: &Statics,
    entities: &mut HashMap<usize, Entity>,
    actor: usize,
    instruction: UseModuleTargeted,
    log: &mut Vec<Log>,
) {
    // First only on origin

    let towards_target = entities
        .get_mut(&actor)
        .and_then(|origin| apply_targeted_to_origin(statics, origin, instruction));

    // Then from origin to target

    let loot = towards_target.and_then(|(origin, targeted, module, free_cargo)| {
        entities
            .get_mut(&(instruction.target_index_in_site as usize))
            .map(|target| {
                log.push(Log::ModuleTargeted((origin, targeted, (&*target).into())));
                apply_targeted_to_target(target, module, free_cargo)
            })
    });

    // And back to origin

    if let Some(loot) = loot {
        if let Some(cargo) = entities.get_mut(&actor).and_then(|entity| match entity {
            Entity::Facility(_) | Entity::Lifeless(_) => None,
            Entity::Npc((_, ship)) | Entity::Player((_, ship)) => Some(&mut ship.cargo),
        }) {
            *cargo = cargo.add(&loot);
        }
    }
}
