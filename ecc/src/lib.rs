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
    pub fn new(a: BigUint, b: BigUint, p: BigUint) -> Self {
        EllipticCurve { a, b, p }
    }

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
    pub fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c + d = r mod p
        let r = c + d;

        // modpow elevetes to a power and then applies to a module
        // r.modpow(exponent, modulus)
        // note: exponent of 1 is always the same number so he does the modulo
        // operation only.
        r.modpow(&BigUint::from(1u32), p)
    }

    pub fn mult(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c * d = r mod p
        let r = c * d;

        r.modpow(&BigUint::from(1u32), p)
    }

    pub fn inv_addition(c: &BigUint, p: &BigUint) -> BigUint {
        // -c mod p
        // c is less than p because we're operating inside the group
        // and shouldn't allow a number to be bigger than the module operation.
        assert!(c < p, "c >= p");
        // format!("number: {} is bigger or equal than p: {}", c, p);
        p - c
    }

    pub fn substract(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let d_inv = FiniteField::inv_addition(d, p);
        FiniteField::add(c, &d_inv, p)
    }

    pub fn inv_multiplication(c: &BigUint, p: &BigUint) -> BigUint {
        // TODO: this function is limited. It use Fermat's Little Theorem and thus
        // it's only is valid for p prime
        // c^(-1) mod p = c^(p-2) mod p
        c.modpow(&(p - BigUint::from(2u32)), p)
    }

    pub fn divide(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let d_inv = FiniteField::inv_multiplication(d, p);
        FiniteField::mult(c, &d_inv, p)
    }
}

