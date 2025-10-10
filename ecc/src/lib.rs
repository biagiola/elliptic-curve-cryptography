use num_bigint::{BigUint};

#[derive(PartialEq, Clone, Debug)]
pub enum Point {
    Coor(BigUint, BigUint),
    Identity,
}

pub struct EllipticCurve {
    // y^2 = x^2 + a * x + b
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl EllipticCurve {
    pub fn add(&self, c: &Point, d: &Point) -> Point {
        assert!(self.is_on_curve(c), "Point is not in curve");
        assert!(self.is_on_curve(d), "Point is not in curve");
        assert!(*c != *d);

        match (c,d) {
            (Point::Identity, _) => d.clone(),
            (_, Point::Identity) => c.clone(),
            (Point::Coor(x1, y1), Point::Coor(x2, y2)) => {
                // println!("{} {}", &y1, &y2);
                // Handle additive inverse: P + (-P) = Identity (vertical line case)
                // the line through them is vertical and doesn't intersect the curve at a third point
                let y1_plus_y2 = FiniteField::add(&y1, &y2, &self.p);
                if x1 == x2 && y1_plus_y2 == BigUint::from(0u32) {
                    return Point::Identity;
                }

                let numerator = FiniteField::substract(y2, y1, &self.p);
                let denominator = FiniteField::substract(x2, x1, &self.p);
                let s = FiniteField::divide(&numerator, &denominator, &self.p);

                let (x3, y3) = self.compute_x3_y3(&x1, &y1, &x2, &s);
                Point::Coor(x3, y3)
            }
        }
    }

    pub fn double(&self, c: &Point) -> Point {
        match c {
            Point::Identity => Point::Identity,
            Point::Coor(x1, y1) => {
                // s = (3 * x1^2 + a) / (2 * y1) mod p
                // x3 = s^2 - 2 * x1 mod p
                // y3 = s(x1 - x3) - y1 mod p

                // x1^2
                let numerator = x1.modpow(&BigUint::from(2u32), &self.p);
                // 3 * x1^2
                let numerator = FiniteField::mult(&BigUint::from(3u32), &numerator, &self.p);
                // 3 * x1^2 + a
                let numerator = FiniteField::add(&self.a, &numerator, &self.p);
                // 2 * y1
                let denominator = FiniteField::mult(&BigUint::from(2u32), &y1, &self.p);
                // (3 * x1^2 + a) / (2 * y1)
                let s = FiniteField::divide(&numerator, &denominator, &self.p);

                let (x3, y3) = self.compute_x3_y3(&x1, &y1, &x1, &s);
                Point::Coor(x3, y3)
            }
        }
    }

    fn compute_x3_y3(
        &self, x1: &BigUint,
        y1: &BigUint,
        x2: &BigUint,
        s: &BigUint
    ) -> (BigUint, BigUint){
        // x3 = s^2 - x1 -x2 mod p
        let s2 = s.modpow(&BigUint::from(2u32), &self.p);
        let s2_minus_x1 = FiniteField::substract(&s2, x1, &self.p);
        let x3 = FiniteField::substract(&s2_minus_x1, x2, &self.p);

        // y3 = x(x1 - x3) - y1 mod p
        let x1_minus_x3 = FiniteField::substract(x1, &x3, &self.p);
        let s_mult_x1_minus_x3 = FiniteField::mult(&s, &x1_minus_x3, &self.p);
        let y3 = FiniteField::substract(&s_mult_x1_minus_x3, &y1, &self.p);

        (x3, y3)
    }

    pub fn scalar_mult(&self, c: &Point, d: &BigUint) -> Point {
        // addition/doubling algorithm.
        // A really big large number like d times the generator A.
        // B = d * A;
        let mut t = c.clone();
        for i in (0..(d.bits() - 1)).rev() {
            t = self.double(&t);
            if d.bit(i) {
                t = self.add(&t, c);
            }
        }
        t
    }

    pub fn is_on_curve(&self, c: &Point) -> bool {
        match c {
            Point::Coor(x, y) => {
                // Curve Formula definition
                // y^2 = x^3 + a * x + b

                // y^2
                let y2 = y.modpow(&BigUint::from(2u32), &self.p);
                // x ^3
                let x3 = x.modpow(&BigUint::from(3u32), &self.p);
                // ax
                let ax = FiniteField::mult(&self.a, x, &self.p);
                // x3 + a * x
                let x3_plus_ax = FiniteField::add(&x3, &ax, &self.p);

                // final comparation
                // y^2 = x^3 + a * x + b
                y2 == FiniteField::add(&x3_plus_ax, &self.b, &self.p)

                // Site note: we cannot use operations directly, we need to use
                // the FiniteField operations
                // let ax = &self.a * x;   // wrong
                // y2 == x3 + ax + &self.b // wrong
            },
            Point::Identity => true
        }
    }
}

pub struct FiniteField {}

impl FiniteField {
    fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c + d = r mod p
        let r = c + d;

        // modpow elevetes to a power and then applies to a module
        // r.modpow(exponent, modulus)
        // note: exponent of 1 is always the same number so he does the modulo
        // operation only.
        r.modpow(&BigUint::from(1u32), p)
    }

    fn mult(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c * d = r mod p
        let r = c * d;

        r.modpow(&BigUint::from(1u32), p)
    }

    fn inv_addition(c: &BigUint, p: &BigUint) -> BigUint {
        // -c mod p
        // c is less than p because we're operating inside the group
        // and shouldn't allow a number to be bigger than the module operation.
        assert!(c < p, "c >= p");
        // format!("number: {} is bigger or equal than p: {}", c, p);
        p - c
    }

    fn substract(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let d_inv = FiniteField::inv_addition(d, p);
        FiniteField::add(c, &d_inv, p)
    }

    fn inv_multiplication(c: &BigUint, p: &BigUint) -> BigUint {
        // TODO: this function is limited. It use Fermat's Little Theorem and thus
        // it's only is valid for p prime
        // c^(-1) mod p = c^(p-2) mod p
        c.modpow(&(p - BigUint::from(2u32)), p)
    }

    fn divide(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let d_inv = FiniteField::inv_multiplication(d, p);
        FiniteField::mult(c, &d_inv, p)
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn test_ec_point_addition() {
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

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
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

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
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

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
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // (5,1) + (5,1) = 2 (5,1) = (6,3)
        let p1 = Point::Coor(BigUint::from(5u32), BigUint::from(1u32));
        let pr = Point::Coor(BigUint::from(6u32), BigUint::from(3u32));

        let res = ec.double(&p1);
        assert_eq!(res, pr);
    }

    #[test]
    fn test_ec_point_doubling_identity() {
        // y^2 = x^3 + 2x + 2 mod 17
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

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
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

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

    #[test]
    fn test_ec_secp256k1() {
        /*
            Curve definition:
            y^2 = x^3 + 7

            p = FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F
            n = FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141
            G = (
                x = 79BE667E F9DCBBAC 55A06295 CE870B07 029BFCDB 2DCE28D9 59F2815B 16F81798
                y = 483ADA77 26A3C465 5DA4FBFC 0E1108A8 FD17B448 A6855419 9C47D08F FB10D4B8
            )
            a = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
            b = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000007
        */

        // n * G = I
        let p = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16
        )
        .expect("could not convert");

        let n = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16
        )
        .expect("could not convert");

        let gx = BigUint::parse_bytes(
            b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16
        )
        .expect("could not convert");

        let gy = BigUint::parse_bytes(
            b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
            16
        )
        .expect("could not convert");

        let ec = EllipticCurve {
            a: BigUint::from(0u32),
            b: BigUint::from(7u32),
            p,
        };

        let g = Point::Coor(gx, gy);
        let res = ec.scalar_mult(&g, &n);

        assert_eq!(res, Point::Identity);

        println!("{:2x?}", &g);
        println!("{:2x?}", ec.scalar_mult(&g, &BigUint(3u32)));
    }
}
