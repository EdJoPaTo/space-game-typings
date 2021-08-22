use std::collections::HashMap;

use crate::fixed::solarsystem::Solarsystem;
use crate::persist::site::Site;

use super::{Entity, Log};

pub fn warp_out(
    solarsystem: Solarsystem,
    entities: &mut HashMap<usize, Entity>,
    actor: usize,
    towards: Site,
    warping_out: &mut Vec<(Solarsystem, Site, Entity)>,
    log: &mut Vec<Log>,
) {
    let entity = entities.get(&actor).expect("only existing actors can warp");
    let ship = match entity {
        Entity::Facility(_) | Entity::Lifeless(_) => {
            unreachable!("Only ships can warp {:?}", entity)
        }
        Entity::Npc((_, ship)) | Entity::Player((_, ship)) => ship,
    };
    if ship.collateral.is_alive() {
        // TODO: capacitor cost
        if let Some(entity) = entities.remove(&actor) {
            log.push(Log::WarpOut((&entity).into()));
            warping_out.push((solarsystem, towards, entity));
        }
    }
}

pub fn jump(
    origin_solarsystem: Solarsystem,
    origin_site: Site,
    entities: &mut HashMap<usize, Entity>,
    actor: usize,
    warping_out: &mut Vec<(Solarsystem, Site, Entity)>,
    log: &mut Vec<Log>,
) {
    if let Site::Stargate(target_solarsystem) = origin_site {
        let can_jump = entities.get(&actor).map_or(false, |entity| match entity {
            Entity::Facility(_) | Entity::Lifeless(_) => {
                unreachable!("Only ships can jump {:?}", entity)
            }
            Entity::Npc((_, ship)) | Entity::Player((_, ship)) => ship.collateral.is_alive(),
        });
        if can_jump {
            if let Some(entity) = entities.remove(&actor) {
                log.push(Log::Jump((&entity).into()));
                warping_out.push((
                    target_solarsystem,
                    Site::Stargate(origin_solarsystem),
                    entity,
                ));
            }
        }
    }
}

pub fn dock(
    solarsystem: Solarsystem,
    site: Site,
    entities: &mut HashMap<usize, Entity>,
    actor: usize,
    docking: &mut Vec<(Solarsystem, u8, Entity)>,
    log: &mut Vec<Log>,
) {
    if let Site::Station(station) = site {
        let can_dock = entities.get(&actor).map_or(false, |entity| match entity {
            Entity::Facility(_) | Entity::Lifeless(_) => {
                unreachable!("Only ships can jump {:?}", entity)
            }
            Entity::Npc((_, ship)) | Entity::Player((_, ship)) => ship.collateral.is_alive(),
        });
        if can_dock {
            if let Some(entity) = entities.remove(&actor) {
                log.push(Log::Dock((&entity).into()));
                docking.push((solarsystem, station, entity));
            }
        }
    }
}
