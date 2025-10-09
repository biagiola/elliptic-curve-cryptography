use num_bigint::{BigUint};

#[derive(PartialEq, Clone, Debug)]
enum Point {
    Coor(BigUint, BigUint),
    Identity,
}

struct EllipticCurve {
    // y^2 = x^2 + a * x + b
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl EllipticCurve {
    fn add(c: &Point, d: &Point) -> Point {
        todo!();
    }

    fn double(c: &Point) -> Point {
        todo!();
    }

    fn scalar_mul(c: &Point, d: &Point) -> Point {
        // addition/doubling algorithm.
        // A really big larga number like d times the generator A.
        // B = d * A;
        todo!();
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

struct FiniteField {}

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

        // p - c
        let c_inv = FiniteField::inv_multiplication(&c, &p);

        // 4 / 3 mod 11 = 12 mod 11 = 1
        assert_eq!(FiniteField::divide(&c, &c, &p), BigUint::from(1u32))
    }
}
