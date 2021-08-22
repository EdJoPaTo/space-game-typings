use crate::entity::Collateral;
use crate::fixed::round_effect::RoundEffect;

const fn apply_damage(mut collateral: Collateral, damage: u16) -> Collateral {
    let structure_dmg = damage.saturating_sub(collateral.armor);
    collateral.armor = collateral.armor.saturating_sub(damage);
    collateral.structure = collateral.structure.saturating_sub(structure_dmg);
    collateral
}

#[allow(clippy::cast_sign_loss)]
const fn can_apply_to_origin(collateral: Collateral, effect: RoundEffect) -> bool {
    match effect {
        RoundEffect::CapacitorDrain(amount) => collateral.capacitor.checked_sub(amount).is_some(),
        RoundEffect::ArmorRepair(_)
        | RoundEffect::CapacitorRecharge(_)
        | RoundEffect::Damage(_)
        | RoundEffect::Mine(_)
        | RoundEffect::StructureRepair(_)
        | RoundEffect::WarpDisruption => true,
    }
}

#[allow(clippy::cast_sign_loss)]
const fn saturating_apply(mut collateral: Collateral, effect: RoundEffect) -> Collateral {
    match effect {
        RoundEffect::CapacitorDrain(amount) => {
            collateral.capacitor = collateral.capacitor.saturating_sub(amount);
            collateral
        }
        RoundEffect::CapacitorRecharge(amount) => {
            collateral.capacitor = collateral.capacitor.saturating_add(amount as u16);
            collateral
        }
        RoundEffect::ArmorRepair(amount) => {
            collateral.armor = collateral.armor.saturating_add(amount);
            collateral
        }
        RoundEffect::StructureRepair(amount) => {
            collateral.structure = collateral.structure.saturating_add(amount);
            collateral
        }
        RoundEffect::Damage(damage) => apply_damage(collateral, damage),
        RoundEffect::Mine(_) | RoundEffect::WarpDisruption => collateral,
    }
}

/// Applies effects to self when possible or returns None.
///
/// Ignores ship limitations! Collateral might have more armor than ship layout can have.
pub fn apply_to_origin(mut collateral: Collateral, effects: &[RoundEffect]) -> Option<Collateral> {
    let can_apply_all = effects.iter().all(|e| can_apply_to_origin(collateral, *e));
    if can_apply_all {
        for effect in effects {
            collateral = saturating_apply(collateral, *effect);
        }
        Some(collateral)
    } else {
        None
    }
}

/// Applies effects to self when possible. Only effects that are possible are applied.
///
/// Ignores ship limitations! Collateral might have more armor than ship layout can have.
pub fn apply_passives(mut collateral: Collateral, effects: &[RoundEffect]) -> Collateral {
    for effects in effects {
        if can_apply_to_origin(collateral, *effects) {
            collateral = saturating_apply(collateral, *effects);
        }
    }
    collateral
}

/// Applies effects in a saturating way. Example: Capacitor 2 - 5 → 0
///
/// Ignores ship limitations! Collateral might have more armor than ship layout can have.
pub fn apply_to_target(mut collateral: Collateral, effects: &[RoundEffect]) -> Collateral {
    for effect in effects {
        collateral = saturating_apply(collateral, *effect);
    }
    collateral
}

#[test]
fn damage_against_armor() {
    let before = Collateral {
        capacitor: 0,
        armor: 42,
        structure: 42,
    };
    assert_eq!(
        apply_damage(before, 10),
        Collateral {
            capacitor: 0,
            armor: 32,
            structure: 42,
        }
    );
}

#[test]
fn damage_against_structure() {
    let before = Collateral {
        capacitor: 0,
        armor: 0,
        structure: 42,
    };
    assert_eq!(
        apply_damage(before, 10),
        Collateral {
            capacitor: 0,
            armor: 0,
            structure: 32,
        }
    );
}

#[test]
fn damage_against_armor_and_structure() {
    let before = Collateral {
        capacitor: 0,
        armor: 3,
        structure: 42,
    };
    assert_eq!(
        apply_damage(before, 10),
        Collateral {
            capacitor: 0,
            armor: 0,
            structure: 35,
        }
    );
}

#[test]
fn damage_against_structure_min_zero() {
    let before = Collateral {
        capacitor: 0,
        armor: 0,
        structure: 2,
    };
    assert_eq!(
        apply_damage(before, 10),
        Collateral {
            capacitor: 0,
            armor: 0,
            structure: 0,
        }
    );
}

#[test]
fn module_with_cap_works_on_origin() {
    let before = Collateral {
        capacitor: 10,
        armor: 0,
        structure: 10,
    };
    let result = apply_to_origin(
        before,
        &[RoundEffect::ArmorRepair(5), RoundEffect::CapacitorDrain(5)],
    );
    assert_eq!(
        result,
        Some(Collateral {
            capacitor: 5,
            armor: 5,
            structure: 10,
        })
    );
}

#[test]
fn module_without_cap_doesnt_work_on_origin() {
    let before = Collateral {
        capacitor: 2,
        armor: 0,
        structure: 10,
    };
    let result = apply_to_origin(
        before,
        &[RoundEffect::ArmorRepair(5), RoundEffect::CapacitorDrain(5)],
    );
    assert_eq!(result, None);
}

#[cfg(test)]
const TEST_DEFAULT_STATUS: Collateral = Collateral {
    capacitor: 10,
    armor: 10,
    structure: 10,
};

#[test]
fn saturating_apply_reduces_capacitor() {
    let result = saturating_apply(TEST_DEFAULT_STATUS, RoundEffect::CapacitorDrain(5));
    assert_eq!(
        result,
        Collateral {
            capacitor: 5,
            armor: 10,
            structure: 10,
        }
    );
}

#[test]
fn saturating_apply_increases_capacitor() {
    let result = saturating_apply(TEST_DEFAULT_STATUS, RoundEffect::CapacitorRecharge(5));
    assert_eq!(
        result,
        Collateral {
            capacitor: 15,
            armor: 10,
            structure: 10,
        }
    );
}

#[test]
fn saturating_apply_increases_armor() {
    let result = saturating_apply(TEST_DEFAULT_STATUS, RoundEffect::ArmorRepair(5));
    assert_eq!(
        result,
        Collateral {
            capacitor: 10,
            armor: 15,
            structure: 10,
        }
    );
}
