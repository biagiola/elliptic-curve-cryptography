use num_bigint::{BigUint};
struct Point {
    x: BigUint,
    y: BigUint,
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
}

struct FiniteField {}

impl FiniteField {
    fn add(c: &BigUint, d: &BigUint) -> BigUint {
        // c + d = r mod p
        todo!()
    }

    fn mult(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c * d = r mod p
        todo!()
    }

    fn inv_add(c: &BigUint, p: &BigUint) -> BigUint {
        // -c mod p
        todo!()
    }

    fn inv_multiplication(c: &BigUint, p: &BigUint) -> BigUint {
        // c^(-1) mod p
        todo!()
    }
}
