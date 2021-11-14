use serde::{Deserialize, Serialize};

mod order;
mod trade;

pub use order::{Order, Trader};
pub use trade::Trade;

/// Keeps all orders for a market of a single item
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct ItemMarket {
    /// Buy orders of buyers wanting to buy items.
    pub buy: Vec<Order>,
    /// Sell orders of sellers wanting to sell items.
    pub sell: Vec<Order>,
}

impl ItemMarket {
    /// Sorts orders.
    /// The first one is the best one for the customer.
    ///
    /// Orders are first sorted by price, then by date.
    /// Older orders are always sorted first.
    pub fn sort(&mut self) {
        self.buy.sort_by_key(|o| o.date);
        self.buy.sort_by(|a, b| b.paperclips.cmp(&a.paperclips));

        self.sell.sort_by_key(|o| o.date);
        self.sell.sort_by(|a, b| a.paperclips.cmp(&b.paperclips));
    }

    /// Removes orders with amount 0
    pub fn cleanup(&mut self) {
        self.buy = self.buy.iter().filter(|o| o.is_valid()).copied().collect();
        self.sell = self.sell.iter().filter(|o| o.is_valid()).copied().collect();
    }

    pub fn resolve(&mut self) -> Vec<Trade> {
        self.sort();
        let mut trades = Vec::new();
        for buy in &mut self.buy {
            for sell in &mut self.sell {
                // Early abort for performance
                if buy.amount == 0 || buy.paperclips < sell.paperclips {
                    break;
                }

                if let Some((trade, remaining_buy, remaining_sell)) = Trade::resolve(*buy, *sell) {
                    *buy = remaining_buy;
                    *sell = remaining_sell;
                    trades.push(trade);
                }
            }
        }
        self.cleanup();
        trades
    }
}

#[test]
fn can_serde_parse_market() {
    let data = ItemMarket {
        buy: vec![Order::example_b(), Order::example_a()],
        sell: vec![Order::example_b()],
    };
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_sort_buy() {
    let mut orders = ItemMarket {
        buy: vec![Order::example_a(), Order::example_b(), Order::example_c()],
        sell: vec![],
    };
    let expected = vec![Order::example_b(), Order::example_a(), Order::example_c()];
    dbg!(&expected);

    orders.sort();
    assert_eq!(orders.sell, vec![]);
    dbg!(&orders.buy);
    assert_eq!(orders.buy, expected);
}

#[test]
fn can_sort_sell() {
    let mut orders = ItemMarket {
        buy: vec![],
        sell: vec![Order::example_a(), Order::example_b(), Order::example_c()],
    };
    let expected = vec![Order::example_c(), Order::example_b(), Order::example_a()];
    dbg!(&expected);

    orders.sort();
    assert_eq!(orders.buy, vec![]);
    dbg!(&orders.sell);
    assert_eq!(orders.sell, expected);
}

#[test]
fn resolve_does_nothing_when_both_empty() {
    let mut orders = ItemMarket::default();
    let trades = orders.resolve();
    assert_eq!(trades, vec![]);
}

#[test]
fn resolve_does_nothing_when_buy_empty() {
    let mut orders = ItemMarket {
        buy: vec![],
        sell: vec![Order::example_a(), Order::example_b(), Order::example_c()],
    };
    let trades = orders.resolve();
    assert_eq!(trades, vec![]);
}

#[test]
fn resolve_does_nothing_when_different_solarsystem() {
    use crate::fixed::solarsystem::Solarsystem;
    let mut buy = Order::example_b();
    buy.solarsystem = Solarsystem::Arama;
    let mut sell = Order::example_b();
    sell.solarsystem = Solarsystem::Iramil;
    let expected = ItemMarket {
        buy: vec![buy],
        sell: vec![sell],
    };
    let mut orders = ItemMarket {
        buy: vec![buy],
        sell: vec![sell],
    };
    let trades = orders.resolve();
    assert_eq!(trades, vec![]);
    assert_eq!(orders, expected);
}

#[test]
fn resolve_exact_single() {
    let mut orders = ItemMarket {
        buy: vec![Order::example_b()],
        sell: vec![Order::example_b()],
    };
    let trades = orders.resolve();
    assert_eq!(trades, vec![Trade::new_test(1337, 666)]);
    assert_eq!(orders, ItemMarket::default());
}

#[test]
fn resolve_exact_with_remaining_sell_orders() {
    let mut orders = ItemMarket {
        buy: vec![Order::example_b()],
        sell: vec![Order::example_b(), Order::example_a()],
    };
    let trades = orders.resolve();
    dbg!(&trades);
    assert_eq!(trades.len(), 1);
    assert_eq!(
        orders,
        ItemMarket {
            buy: vec![],
            sell: vec![Order::example_a()],
        }
    );
}

#[test]
fn resolve_exact_with_remaining_buy_orders() {
    let mut orders = ItemMarket {
        buy: vec![Order::example_b(), Order::example_a()],
        sell: vec![Order::example_b()],
    };
    let trades = orders.resolve();
    dbg!(&trades);
    assert_eq!(trades.len(), 1);
    assert_eq!(
        orders,
        ItemMarket {
            buy: vec![Order::example_a()],
            sell: vec![],
        }
    );
}

#[test]
fn resolve_partial() {
    let mut orders = ItemMarket {
        buy: vec![Order::example_a()],
        sell: vec![Order::example_b()],
    };
    let trades = orders.resolve();
    assert_eq!(trades, vec![Trade::new_test(42, 666)]);
    assert_eq!(
        orders,
        ItemMarket {
            buy: vec![],
            sell: vec![Order::example_b().reduce_to(1295)],
        }
    );
}

#[test]
fn resolve_exact_with_better_wrong_locations() {
    use crate::fixed::solarsystem::Solarsystem;
    let mut orders = ItemMarket {
        buy: vec![
            Order::new_test("1997-12-19T16:53:14Z", Solarsystem::Arama, 1, 1337),
            Order::new_test("1997-12-19T16:53:14Z", Solarsystem::default(), 20, 5),
        ],
        sell: vec![
            Order::new_test("1997-12-19T16:53:14Z", Solarsystem::Iramil, 1337, 1),
            Order::new_test("1997-12-19T16:53:14Z", Solarsystem::default(), 20, 5),
        ],
    };
    let trades = orders.resolve();
    assert_eq!(trades, vec![Trade::new_test(20, 5)]);
    assert_eq!(
        orders,
        ItemMarket {
            buy: vec![Order::new_test(
                "1997-12-19T16:53:14Z",
                Solarsystem::Arama,
                1,
                1337
            ),],
            sell: vec![Order::new_test(
                "1997-12-19T16:53:14Z",
                Solarsystem::Iramil,
                1337,
                1
            ),],
        }
    );
}
