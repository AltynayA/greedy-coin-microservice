use rust_axum_greedy_coin_microservice::greedy_coin_change;
use rust_decimal::Decimal;

#[test]
fn test_zero_amount() {
    assert_eq!(greedy_coin_change(Decimal::new(0, 0)), vec![] as Vec<Decimal>);
}

#[test]
fn test_exact_coins() {
    assert_eq!(greedy_coin_change(Decimal::new(1, 0)), vec![Decimal::new(1, 0)]);
    assert_eq!(greedy_coin_change(Decimal::new(5, 0)), vec![Decimal::new(5, 0)]);
    assert_eq!(greedy_coin_change(Decimal::new(10, 0)), vec![Decimal::new(10, 0)]);
    assert_eq!(greedy_coin_change(Decimal::new(25, 0)), vec![Decimal::new(25, 0)]);
}

#[test]
fn test_small_amounts() {
    assert_eq!(greedy_coin_change(Decimal::new(6, 0)),  vec![Decimal::new(5, 0), Decimal::new(1, 0)]);
    assert_eq!(greedy_coin_change(Decimal::new(11, 0)), vec![Decimal::new(10, 0), Decimal::new(1, 0)]);
    assert_eq!(greedy_coin_change(Decimal::new(16, 0)), vec![Decimal::new(10, 0), Decimal::new(5, 0), Decimal::new(1, 0)]);
    assert_eq!(greedy_coin_change(Decimal::new(26, 0)), vec![Decimal::new(25, 0), Decimal::new(1, 0)]);
    assert_eq!(greedy_coin_change(Decimal::new(27, 0)), vec![Decimal::new(25, 0), Decimal::new(1, 0), Decimal::new(1, 0)]);
}

#[test]
fn test_negative_amounts() {
    assert_eq!(greedy_coin_change(Decimal::new(-6, 0)),  vec![Decimal::new(5, 0), Decimal::new(1, 0)]);
    assert_eq!(greedy_coin_change(Decimal::new(-30, 0)), vec![Decimal::new(25, 0), Decimal::new(5, 0)]);
    assert_eq!(greedy_coin_change(Decimal::new(-27, 0)), vec![Decimal::new(25, 0), Decimal::new(1, 0), Decimal::new(1, 0)]);
}

#[test]
fn test_large_amounts() {
    let result = greedy_coin_change(Decimal::new(1000, 0));
    let sum: Decimal = result.iter().sum();
    assert_eq!(sum, Decimal::new(1000, 0));

    let result_neg = greedy_coin_change(Decimal::new(-1000, 0));
    let sum_neg: Decimal = result_neg.iter().sum();
    assert_eq!(sum_neg, Decimal::new(1000, 0));
}

#[test]
fn test_fractional_cents_rounding() {
    let amt = Decimal::new(125, 1); // 12.5 -> 13
    let result = greedy_coin_change(amt);
    let sum: Decimal = result.iter().sum();
    assert_eq!(sum, Decimal::new(13, 0));
}

