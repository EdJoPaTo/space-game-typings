use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::fixed::item::Item;

type Amount = u32;

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Storage(Vec<(Item, Amount)>);

#[cfg(feature = "typescript")]
ts_rs::export! {
    Storage => "storage.ts",
}

impl Serialize for Storage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let map: HashMap<Item, Amount> = self.into();
        let mut ordered = map
            .iter()
            .filter(|(_item, amount)| **amount > 0)
            .collect::<Vec<_>>();
        ordered.sort();
        ordered.serialize(serializer)
    }
}

impl From<HashMap<Item, Amount>> for Storage {
    fn from(map: HashMap<Item, Amount>) -> Self {
        let mut result = Vec::new();
        for (item, amount) in map {
            if amount > 0 {
                result.push((item, amount));
            }
        }
        Self(result)
    }
}

impl From<&Storage> for HashMap<Item, Amount> {
    fn from(storage: &Storage) -> Self {
        let mut result: HashMap<Item, Amount> = HashMap::new();
        for (item, amount) in &storage.0 {
            let total = result.entry(*item).or_default();
            *total += amount;
        }
        result
    }
}

impl From<Vec<Item>> for Storage {
    fn from(items: Vec<Item>) -> Self {
        let mut result: HashMap<Item, Amount> = HashMap::new();
        for item in items {
            let amount = result.entry(item).or_default();
            *amount += 1;
        }
        result.into()
    }
}

impl Storage {
    #[must_use]
    pub fn single(item: Item, amount: u32) -> Self {
        Self(vec![(item, amount)])
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        for (_, amount) in &self.0 {
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
        for (_, amount) in &self.0 {
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

    #[must_use]
    pub fn saturating_add(&self, item: Item, amount: Amount) -> Self {
        let mut map: HashMap<Item, Amount> = self.into();
        let entry = map.entry(item).or_default();
        *entry = entry.saturating_add(amount);
        map.into()
    }

    #[must_use]
    pub fn checked_sub(&self, item: Item, amount: Amount) -> Option<Self> {
        let mut map: HashMap<Item, Amount> = self.into();
        let entry = map.entry(item).or_default();
        #[allow(clippy::option_if_let_else)]
        if let Some(new_amount) = entry.checked_sub(amount) {
            *entry = new_amount;
            Some(map.into())
        } else {
            None
        }
    }
}

#[test]
fn can_parse_storage() {
    use crate::fixed::item::Ore;
    use crate::fixed::module::targeted::Targeted;
    let data = Storage(vec![
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
        (Item::Ore(Ore::Aromit), 12),
    ]);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn serializing_storage_simplifies_it() {
    use crate::fixed::item::Ore;
    use crate::fixed::module::passive::Passive;
    use crate::fixed::module::targeted::Targeted;
    let data = Storage(vec![
        (Item::Ore(Ore::Aromit), 12),
        (Item::ModulePassive(Passive::RookieArmorPlate), 0),
        (Item::Ore(Ore::Aromit), 8),
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
    ]);
    let expected = Storage(vec![
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
        (Item::Ore(Ore::Aromit), 20),
    ]);
    let parsed = crate::test_helper::json_parsed(&data);
    assert_eq!(parsed, expected);
}

#[test]
fn can_add() {
    use crate::fixed::item::Ore;
    use crate::fixed::module::targeted::Targeted;
    let data = Storage(vec![
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
        (Item::Ore(Ore::Aromit), 12),
    ]);
    let result = data.saturating_add(Item::Ore(Ore::Aromit), 8);
    assert_eq!(
        result.amount(Item::ModuleTargeted(Targeted::RookieLaser)),
        2
    );
    assert_eq!(result.amount(Item::Ore(Ore::Aromit)), 20);
}

#[test]
fn can_sub() {
    use crate::fixed::item::Ore;
    use crate::fixed::module::targeted::Targeted;
    let data = Storage(vec![
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
        (Item::Ore(Ore::Aromit), 12),
    ]);
    if let Some(result) = data.checked_sub(Item::Ore(Ore::Aromit), 2) {
        assert_eq!(
            result.amount(Item::ModuleTargeted(Targeted::RookieLaser)),
            2
        );
        assert_eq!(result.amount(Item::Ore(Ore::Aromit)), 10);
    } else {
        panic!("should work");
    }
}

#[test]
fn can_not_sub_when_not_there() {
    use crate::fixed::item::Ore;
    use crate::fixed::module::targeted::Targeted;
    let data = Storage(vec![(Item::ModuleTargeted(Targeted::RookieLaser), 2)]);
    let result = data.checked_sub(Item::Ore(Ore::Aromit), 2);
    assert!(result.is_none());
}

#[test]
fn can_not_sub_when_not_enough() {
    use crate::fixed::item::Ore;
    use crate::fixed::module::targeted::Targeted;
    let data = Storage(vec![
        (Item::ModuleTargeted(Targeted::RookieLaser), 2),
        (Item::Ore(Ore::Aromit), 2),
    ]);
    let result = data.checked_sub(Item::Ore(Ore::Aromit), 10);
    assert!(result.is_none());
}
