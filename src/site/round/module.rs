use std::collections::HashMap;

use crate::entity::Collateral;
use crate::fixed::item::Item;
use crate::fixed::module::Targeted;
use crate::fixed::round_effect::RoundEffect;
use crate::fixed::{module, Statics};
use crate::site::{Actor, Entity, Log};

use super::effect::{apply_to_origin, apply_to_target};

pub fn self_destruct(entity: &mut Entity) {
    let collateral = match entity {
        Entity::Facility(_) => unreachable!("a facility cant self destruct"),
        Entity::Asteroid(info) => &mut info.collateral,
        Entity::Npc((_, ship)) | Entity::Player((_, ship)) => &mut ship.collateral,
    };
    *collateral = Collateral::DEAD;
}

pub fn apply_untargeted(statics: &Statics, entity: &mut Entity, module_index: u8) {
    let ship = match entity {
        Entity::Facility(_) | Entity::Asteroid(_) => {
            unreachable!("Only ships can use modules {:?}", entity)
        }
        Entity::Npc((_, ship)) | Entity::Player((_, ship)) => ship,
    };
    if let Some(module) = ship
        .fitting
        .slots_untargeted
        .get(module_index as usize)
        .map(|o| statics.modules_untargeted.get(o))
    {
        if let Some(result) = apply_to_origin(ship.collateral, &module.effects) {
            ship.collateral = result;
        }
    } else {
        println!(
            "WARN: untargeted module not found on ship {} {:?}",
            module_index, ship.fitting
        );
    }
}

#[must_use]
fn apply_targeted_to_origin<'s>(
    statics: &'s Statics,
    entity: &mut Entity,
    module_index: u8,
) -> Option<(Actor, Targeted, &'s module::TargetedDetails, u32)> {
    let ship = match entity {
        Entity::Facility(_) | Entity::Asteroid(_) => {
            unreachable!("Only ships can use modules {:?}", entity)
        }
        Entity::Npc((_, ship)) | Entity::Player((_, ship)) => ship,
    };
    if let Some(targeted) = ship
        .fitting
        .slots_targeted
        .get(module_index as usize)
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
            "WARN: targeted module not found on ship {} {:?}",
            module_index, ship.fitting
        );
    }
    None
}

/// Returns loot
#[must_use]
fn apply_targeted_to_target(
    entity: &mut Entity,
    module: &module::TargetedDetails,
    free_cargo: u32,
) -> Vec<(Item, u32)> {
    match entity {
        Entity::Facility(_) => {
            // immune
            vec![]
        }
        Entity::Asteroid(entity) => {
            entity.collateral = apply_to_target(entity.collateral, &module.effects_target);
            let mut loot = Vec::new();
            let amount_mined = module
                .effects_target
                .iter()
                .find_map(|o| match o {
                    RoundEffect::Mine(amount) => Some(*amount),
                    _ => None,
                })
                .unwrap_or_default()
                .min(free_cargo)
                .min(entity.remaining_ore);
            entity.remaining_ore -= amount_mined;
            loot.push((Item::Ore(entity.ore), amount_mined));
            loot
        }
        Entity::Npc((_, ship)) | Entity::Player((_, ship)) => {
            ship.collateral = apply_to_target(ship.collateral, &module.effects_target);
            vec![]
        }
    }
}

pub fn apply_targeted(
    statics: &Statics,
    entities: &mut HashMap<usize, Entity>,
    actor: usize,
    module_index: u8,
    target_index_in_site: u8,
    log: &mut Vec<Log>,
) {
    // First only on origin

    let towards_target = entities
        .get_mut(&actor)
        .and_then(|origin| apply_targeted_to_origin(statics, origin, module_index));

    // Then from origin to target

    let loot = towards_target.and_then(|(origin, targeted, module, free_cargo)| {
        entities
            .get_mut(&(target_index_in_site as usize))
            .map(|target| {
                log.push(Log::ModuleTargeted((origin, targeted, (&*target).into())));
                apply_targeted_to_target(target, module, free_cargo)
            })
    });

    // And back to origin

    if let Some(loot) = loot {
        if let Some(cargo) = entities.get_mut(&actor).and_then(|entity| match entity {
            Entity::Facility(_) | Entity::Asteroid(_) => None,
            Entity::Npc((_, ship)) | Entity::Player((_, ship)) => Some(&mut ship.cargo),
        }) {
            cargo.append(&mut loot.into());
        }
    }
}
