use crate::{EllipticCurve, FiniteField, Point};
use num_bigint::{BigUint, RandBigInt};
use rand::{self, Rng};
use sha256::{digest, try_digest};

pub struct ECDSA {
    elliptic_curve: EllipticCurve,
    a_gen: Point,
    q_order: BigUint,
}

impl ECDSA {
    // Generate: d, B where B = d A
    pub fn generate_key_pair(&self) -> (BigUint, Point) {
        let priv_key = self.generate_priv_key();
        let pub_key = self.generate_pub_key(&priv_key);
        (priv_key, pub_key)
    }

    pub fn generate_priv_key(&self) -> BigUint {
        self.generate_random_positive_number_less_than(&self.q_order)
    }

    pub fn generate_pub_key(&self, priv_key: &BigUint) -> Point {
        self.elliptic_curve.scalar_mult(&self.a_gen, priv_key)
    }

    // (0, max)
    pub fn generate_random_positive_number_less_than(&self, max: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();
        rng.gen_biguint_range(&BigUint::from(1u32), &max)
    }

    /// R = k A -? take `r = x` component
    /// s = (hash(message) + d * r) * k^(-1) mod q
    ///
    pub fn sign(
        &self,
        hash: &BigUint,
        priv_key: &BigUint,
        k_random: &BigUint,
    ) -> (BigUint, BigUint) {
        assert!(
            *hash < self.q_order,
            "Hash is bigger than the order of the EC group"
        );
        assert!(
            *priv_key < self.q_order,
            "Private key is bigger than the order of the EC group"
        );
        assert!(
            *k_random < self.q_order,
            "Random number `k` is bigger than the order of the EC group"
        );

        let r_point: Point = self.elliptic_curve.scalar_mult(&self.a_gen, k_random);

        if let Point::Coor(r, _) = r_point {
            let s = FiniteField::mult(&r, priv_key, &self.q_order);
            let s = FiniteField::add(&s, hash, &self.q_order);
            let k_inv = FiniteField::inv_multiplication(k_random, &self.q_order);
            let s = FiniteField::mult(&s, &k_inv, &self.q_order);

            return (r, s);
        }

        panic!("The random point R should not be the identity");
    }

    pub fn verify(&self, hash: &BigUint, pub_key: Point, signature: &(BigUint, BigUint)) -> bool {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sign_verify() {
        let elliptic_curve = EllipticCurve::new(
            BigUint::from(2u32),
            BigUint::from(2u32),
            BigUint::from(17u32),
        );

        let a_gen = Point::Coor(BigUint::from(5u32), BigUint::from(1u32));

        let q_order = BigUint::from(19u32);

        let ecdsa = ECDSA {
            elliptic_curve,
            a_gen,
            q_order
        };

        let priv_key = BigUint::from(7u32);
        let pub_key = ecdsa.generate_pub_key(&priv_key);

        let hash = BigUint::from(10u32);
        let k_random = BigUint::from(18u32);

        let signature = ecdsa.sign(&hash, &priv_key, &k_random);

        println!("signature: {:?}", signature);
    }
}
