use ecc::{EllipticCurve, Point};
use num_bigint::BigUint;

#[test]
fn test_ec_point_addition() {
    let ec = EllipticCurve::new(
        BigUint::from(2u32),
        BigUint::from(2u32),
        BigUint::from(17u32),
    );

    // (6,3) + (5,1) = (10,6)
    let p1 = Point::Coor(BigUint::from(6u32), BigUint::from(3u32));
    let p2 = Point::Coor(BigUint::from(5u32), BigUint::from(1u32));
    let pr = Point::Coor(BigUint::from(10u32), BigUint::from(6u32));

    let res = ec.add(&p1, &p2);
    assert_eq!(res, pr);

    // commutative rule of a abelian group should be satisfy
    let res = ec.add(&p2, &p1);
    assert_eq!(res, pr);
}

#[test]
fn test_ec_point_addition_identity() {
    let ec = EllipticCurve::new(
        BigUint::from(2u32),
        BigUint::from(2u32),
        BigUint::from(17u32),
    );

    // (6,3) + I = (10,6)
    let p1 = Point::Coor(BigUint::from(6u32), BigUint::from(3u32));
    let p2 = Point::Identity;
    let pr = p1.clone();

    let res = ec.add(&p1, &p2);
    assert_eq!(res, pr);

    let res = ec.add(&p2, &p1);
    assert_eq!(res, pr);
}

#[test]
fn test_ec_point_addition_reflected_in_x() {
    let ec = EllipticCurve::new(
        BigUint::from(2u32),
        BigUint::from(2u32),
        BigUint::from(17u32),
    );

    // (5,16) + (5,1) = Point::Identity
    let p1 = Point::Coor(BigUint::from(5u32), BigUint::from(16u32));
    let p2 = Point::Coor(BigUint::from(5u32), BigUint::from(1u32));
    let pr = Point::Identity;

    let res = ec.add(&p1, &p2);
    assert_eq!(res, pr);

    // commutative rule of a abelian group should be satisfy
    let res = ec.add(&p2, &p1);
    assert_eq!(res, pr);
}

#[test]
fn test_ec_point_doubling() {
    // y^2 = x^3 + 2x + 2 mod 17
    let ec = EllipticCurve::new(
        BigUint::from(2u32),
        BigUint::from(2u32),
        BigUint::from(17u32),
    );

    // (5,1) + (5,1) = 2 (5,1) = (6,3)
    let p1 = Point::Coor(BigUint::from(5u32), BigUint::from(1u32));
    let pr = Point::Coor(BigUint::from(6u32), BigUint::from(3u32));

    let res = ec.double(&p1);
    assert_eq!(res, pr);
}

#[test]
fn test_ec_point_doubling_identity() {
    // y^2 = x^3 + 2x + 2 mod 17
    let ec = EllipticCurve::new(
        BigUint::from(2u32),
        BigUint::from(2u32),
        BigUint::from(17u32),
    );

    // I + I = 2 I = I
    let p1 = Point::Identity;
    let pr = Point::Identity;

    let res = ec.double(&p1);
    assert_eq!(res, pr);
}

#[test]
fn test_bits() {
    let a = BigUint::from(2u32);
    assert!(!a.bit(0));
    assert!(a.bit(1));
}

#[test]
fn test_ec_point_scalar_multiplication() {
    // y^2 = x^3 + 2x + 2 mod 17
    // the order of this group is 19 and every point
    // |G| = 19
    // btw groups that have order, which is a prime, means that every
    // point in the curve is a generator.
    // If we apply 19 * A = I where A is any point in the curve
    let ec = EllipticCurve::new(
        BigUint::from(2u32),
        BigUint::from(2u32),
        BigUint::from(17u32),
    );

    let c = Point::Coor(BigUint::from(5u32), BigUint::from(1u32));

    // 2 (5,1) = (6,3)
    let pr = Point::Coor(BigUint::from(6u32), BigUint::from(3u32));
    let res = ec.scalar_mult(&c, &BigUint::from(2u32));
    assert_eq!(res, pr);

    // 10 (5,1) = (7,11)
    let pr = Point::Coor(BigUint::from(7u32), BigUint::from(11u32));
    let res = ec.scalar_mult(&c, &BigUint::from(10u32));
    assert_eq!(res, pr);

    // 16 (5,1) = (10,11)
    let pr = Point::Coor(BigUint::from(10u32), BigUint::from(11u32));
    let res = ec.scalar_mult(&c, &BigUint::from(16u32));
    assert_eq!(res, pr);

    // 17 (5,1) = (6,14)
    let pr = Point::Coor(BigUint::from(6u32), BigUint::from(14u32));
    let res = ec.scalar_mult(&c, &BigUint::from(17u32));
    assert_eq!(res, pr);

    // 18 (5,1) = (5,16)
    let pr = Point::Coor(BigUint::from(5u32), BigUint::from(16u32));
    let res = ec.scalar_mult(&c, &BigUint::from(18u32));
    assert_eq!(res, pr);

    // 19 (5,1) = I
    let pr = Point::Identity;
    let res = ec.scalar_mult(&c, &BigUint::from(19u32));
    assert_eq!(res, pr);
}
