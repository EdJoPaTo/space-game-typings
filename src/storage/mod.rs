use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::fixed::item::Item;

type Amount = u32;

// TODO: remove Clone in order to ensure Items only get moved?

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Storage(HashMap<Item, Amount>);

#[cfg(feature = "typescript")]
ts_rs::export! {
    Storage => "typescript/generated-storage.ts"
}

impl Serialize for Storage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let ordered = self
            .0
            .iter()
            .filter(|(_item, amount)| **amount > 0)
            .collect::<BTreeMap<_, _>>();
        ordered.serialize(serializer)
    }
}

impl From<Vec<(Item, u32)>> for Storage {
    fn from(items: Vec<(Item, u32)>) -> Self {
        let mut result: HashMap<Item, Amount> = HashMap::new();
        for (item, i) in items {
            let amount = result.entry(item).or_default();
            *amount += i;
        }
        Self(result)
    }
}

impl From<Vec<Item>> for Storage {
    fn from(items: Vec<Item>) -> Self {
        let mut result: HashMap<Item, Amount> = HashMap::new();
        for item in items {
            let amount = result.entry(item).or_default();
            *amount += 1;
        }
        Self(result)
    }
}

impl Storage {
    #[must_use]
    pub fn new_single(item: Item, amount: u32) -> Self {
        let mut result = HashMap::new();
        result.insert(item, amount);
        Self(result)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        for amount in self.0.values() {
            if amount > &0 {
                return false;
            }
        }
        true
    }

    /// Total slots used by the stored goods.
    /// Will probably be replaced in the future with a `total_volume` or something like that.
    #[must_use]
    pub fn total_slots(&self) -> Amount {
        let mut total: Amount = 0;
        for amount in self.0.values() {
            total = amount.saturating_add(total);
        }
        total
    }

    #[must_use]
    pub fn amount(&self, item: Item) -> Amount {
        let mut total: Amount = 0;
        for (current_item, amount) in &self.0 {
            if current_item == &item {
                total = amount.saturating_add(total);
            }
        }
        total
    }

    pub fn saturating_add(&mut self, item: Item, amount: Amount) {
        let entry = self.0.entry(item).or_default();
        *entry = entry.saturating_add(amount);
    }

    /// Takes the wanted items.
    /// # Returns
    /// Returns true when all wanted items were taken. Returns false when there are not enough items.
    #[must_use]
    pub fn take_exact(&mut self, item: Item, amount: Amount) -> bool {
        let entry = self.0.entry(item).or_default();
        let possible = *entry >= amount;
        if possible {
            *entry -= amount;
        }
        possible
    }

    /// Takes as many items as possible.
    /// # Returns
    /// The amount of items that were taken.
    #[must_use]
    pub fn take_max(&mut self, item: Item, amount: Amount) -> Amount {
        let entry = self.0.entry(item).or_default();
        let possible = amount.min(*entry);
        *entry -= possible;
        possible
    }

    /// Moves all items from `other` to `self`.
    pub fn append(&mut self, other: &mut Storage) {
        for (item, amount) in &other.0 {
            self.saturating_add(*item, *amount);
        }
        other.0.clear();
    }

    #[must_use]
    pub fn to_vec(&self) -> Vec<(Item, u32)> {
        self.0
            .iter()
            .map(|(item, amount)| (*item, *amount))
            .collect()
    }
}

#[test]
fn can_parse_storage() {
    use crate::fixed::item::Ore;
    use crate::fixed::module::targeted::Targeted;
    let data: Storage = vec![
        Item::ModuleTargeted(Targeted::RookieLaser),
        Item::Ore(Ore::Aromit),
        Item::Ore(Ore::Aromit),
    ]
    .into();
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn serializing_storage_simplifies_it() {
    use crate::fixed::item::Ore;
    use crate::fixed::module::passive::Passive;
    use crate::fixed::module::targeted::Targeted;
    let data: Storage = vec![
        (Item::Ore(Ore::Aromit), 12),
        (Item::ModulePassive(Passive::RookieArmorPlate), 0),
        (Item::Ore(Ore::Aromit), 8),
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
    ]
    .into();
    let expected: Storage = vec![
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
        (Item::Ore(Ore::Aromit), 20),
    ]
    .into();
    let parsed = crate::test_helper::json_parsed(&data);
    assert_eq!(parsed, expected);
}

#[test]
fn can_add() {
    use crate::fixed::item::Ore;
    use crate::fixed::module::targeted::Targeted;
    let mut data: Storage = vec![
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
        (Item::Ore(Ore::Aromit), 12),
    ]
    .into();
    let expected: Storage = vec![
        (Item::Ore(Ore::Aromit), 20),
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
    ]
    .into();
    data.saturating_add(Item::Ore(Ore::Aromit), 8);
    assert_eq!(data, expected);
}

#[test]
fn take_exact_works() {
    use crate::fixed::item::Ore;
    use crate::fixed::module::targeted::Targeted;
    let mut data: Storage = vec![
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
        (Item::Ore(Ore::Aromit), 12),
    ]
    .into();
    let expected: Storage = vec![
        (Item::Ore(Ore::Aromit), 10),
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
    ]
    .into();
    let worked = data.take_exact(Ore::Aromit.into(), 2);
    assert!(worked);
    assert_eq!(data, expected);
}

#[test]
fn takes_exact_takes_nothing_when_empty() {
    use crate::fixed::item::Ore;
    use crate::fixed::module::targeted::Targeted;
    let mut data: Storage = vec![
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
        (Item::Ore(Ore::Aromit), 0),
    ]
    .into();
    let expected: Storage = vec![
        (Item::Ore(Ore::Aromit), 0),
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
    ]
    .into();
    let worked = data.take_exact(Ore::Aromit.into(), 2);
    assert!(!worked);
    assert_eq!(data, expected);
}

#[test]
fn takes_exact_takes_nothing_when_not_enough() {
    use crate::fixed::item::Ore;
    use crate::fixed::module::targeted::Targeted;
    let mut data: Storage = vec![
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
        (Item::Ore(Ore::Aromit), 2),
    ]
    .into();
    let expected: Storage = vec![
        (Item::Ore(Ore::Aromit), 2),
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
    ]
    .into();
    let worked = data.take_exact(Ore::Aromit.into(), 10);
    assert!(!worked);
    assert_eq!(data, expected);
}

#[test]
fn take_max_takes_all() {
    use crate::fixed::item::Ore;
    use crate::fixed::module::targeted::Targeted;
    let mut data: Storage = vec![
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
        (Item::Ore(Ore::Aromit), 12),
    ]
    .into();
    let expected: Storage = vec![
        (Item::Ore(Ore::Aromit), 10),
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
    ]
    .into();
    let took = data.take_max(Ore::Aromit.into(), 2);
    assert_eq!(took, 2);
    assert_eq!(data, expected);
}

#[test]
fn takes_max_nothing_when_empty() {
    use crate::fixed::item::Ore;
    use crate::fixed::module::targeted::Targeted;
    let mut data: Storage = vec![
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
        (Item::Ore(Ore::Aromit), 0),
    ]
    .into();
    let expected: Storage = vec![
        (Item::Ore(Ore::Aromit), 0),
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
    ]
    .into();
    let took = data.take_max(Ore::Aromit.into(), 2);
    assert_eq!(took, 0);
    assert_eq!(data, expected);
}

#[test]
fn takes_max_partial() {
    use crate::fixed::item::Ore;
    use crate::fixed::module::targeted::Targeted;
    let mut data: Storage = vec![
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
        (Item::Ore(Ore::Aromit), 2),
    ]
    .into();
    let expected: Storage = vec![
        (Item::Ore(Ore::Aromit), 0),
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
    ]
    .into();
    let took = data.take_max(Ore::Aromit.into(), 10);
    assert_eq!(took, 2);
    assert_eq!(data, expected);
}
