/* Greedy Coin Change Logic*/
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;

pub fn greedy_coin_change(amount: Decimal) -> Vec<Decimal> {
    let mut coins = vec![
        Decimal::new(25, 0),
        Decimal::new(10, 0),
        Decimal::new(5, 0),
        Decimal::new(1, 0),
    ];
    coins.sort();
    coins.reverse();

    let mut change = vec![];
    // handle negative amounts and round
    let mut remaining = amount.abs()
        .round_dp_with_strategy(0, RoundingStrategy::MidpointAwayFromZero);

    for coin in coins {
        while remaining >= coin {
            remaining -= coin;
            change.push(coin);
        }
    }

    change
}

