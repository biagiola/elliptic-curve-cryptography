use ecc::{FiniteField};
use num_bigint::BigUint;

#[test]
fn test_add_1() {
    let c = BigUint::from(4u32);
    let d = BigUint::from(10u32);
    let p = BigUint::from(11u32);

    // 4 * 10 mod 11 = 3
    let r = FiniteField::add(&c, &d, &p);

    assert_eq!(r, BigUint::from(3u32));
}

#[test]
fn test_add_2() {
    let c = BigUint::from(4u32);
    let d = BigUint::from(10u32);
    let p = BigUint::from(31u32);

    // 4 * 10 mod 11 = 3
    let r = FiniteField::add(&c, &d, &p);

    assert_eq!(r, BigUint::from(14u32));
}

#[test]
fn test_mult_1() {
    let c = BigUint::from(4u32);
    let d = BigUint::from(10u32);
    let p = BigUint::from(11u32);

    // 4 * 10 mod 11 = 3
    let r = FiniteField::mult(&c, &d, &p);

    assert_eq!(r, BigUint::from(7u32));
}

#[test]
fn test_mult_2() {
    let c = BigUint::from(4u32);
    let d = BigUint::from(10u32);
    let p = BigUint::from(51u32);

    // 4 * 10 mod 11 = 3
    let r = FiniteField::mult(&c, &d, &p);

    assert_eq!(r, BigUint::from(40u32));
}

#[test]
fn test_inv_addition_1() {
    let c = BigUint::from(4u32);
    let p = BigUint::from(51u32);

    // p - c
    let r = FiniteField::inv_addition(&c, &p);

    assert_eq!(r, BigUint::from(47u32));
}

#[test]
#[should_panic]
fn test_inv_addition_2() {
    let c = BigUint::from(52u32);
    let p = BigUint::from(51u32);

    // p - c
    FiniteField::inv_addition(&c, &p);
}

#[test]
fn test_inv_addition_identity() {
    let c = BigUint::from(4u32);
    let p = BigUint::from(51u32);

    // p - c
    let c_inv = FiniteField::inv_addition(&c, &p);

    assert_eq!(c_inv, BigUint::from(47u32));
    assert_eq!(FiniteField::add(&c, &c_inv, &p), BigUint::from(0u32))
    // 0 is the identity element in an addition
}

#[test]
fn test_substract() {
    let c = BigUint::from(4u32);
    let p = BigUint::from(51u32);

    assert_eq!(FiniteField::substract(&c, &c, &p), BigUint::from(0u32))
}

#[test]
fn test_inv_multiplication_identity() {
    let c = BigUint::from(4u32);
    let p = BigUint::from(11u32);

    // p - c
    let c_inv = FiniteField::inv_multiplication(&c, &p);

    // 4 * 3 mod 11 = 12 mod 11 = 1
    assert_eq!(c_inv, BigUint::from(3u32));
    assert_eq!(FiniteField::mult(&c, &c_inv, &p), BigUint::from(1u32))
    // 1 is the identity element in an multiplication
}

#[test]
fn test_divide() {
    let c = BigUint::from(4u32);
    let p = BigUint::from(11u32);

    // 4 / 3 mod 11 = 12 mod 11 = 1
    assert_eq!(FiniteField::divide(&c, &c, &p), BigUint::from(1u32))
}
