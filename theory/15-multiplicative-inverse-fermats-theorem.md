# Elliptic Curve Cryptography - Course Notes

## Theory

### Lesson 15: Multiplicative Inverse using Fermat's Little Theorem

This lesson explains how to efficiently compute multiplicative inverses in finite fields when working with prime moduli—a critical operation for ECDSA signature verification.

#### The Multiplicative Inverse

The **multiplicative inverse** of `a` modulo `p` is a number `a⁻¹` such that:

`a × a⁻¹ ≡ 1 (mod p)`

**Example:** If `p = 7` and `a = 3`:
- `3 × 5 = 15 ≡ 1 (mod 7)`
- Therefore, `3⁻¹ = 5` in the finite field `𝔽₇`

#### Finite Fields (Brief)

A **finite field** `𝔽ₚ` (where `p` is prime) is the set of integers `{0, 1, 2, ..., p-1}` with addition and multiplication operations performed modulo `p`. In a finite field:
- Every non-zero element has a multiplicative inverse
- All arithmetic operations stay within the field
- The field has exactly `p` elements

This structure is essential for elliptic curve cryptography because it provides a well-defined mathematical framework for all curve operations.

#### Fermat's Little Theorem

**Fermat's Little Theorem** states that for any integer `a` and prime `p` where `gcd(a, p) = 1`:

`a^(p-1) ≡ 1 (mod p)`

Rearranging this equation:

`a × a^(p-2) ≡ 1 (mod p)`

This reveals that: **`a⁻¹ ≡ a^(p-2) (mod p)`**

So instead of using complex algorithms to find the inverse, we can simply compute `a^(p-2) mod p` when `p` is prime!

**Why we use it:** Fermat's Little Theorem provides an elegant and efficient way to compute multiplicative inverses in prime finite fields. The `modpow` function uses fast exponentiation, making this computation feasible even for cryptographic-sized numbers (256+ bits).

#### The Implementation

```rust
pub fn inv_mult_prime(a: &BigUint, p: &BigUint) -> Result<BigUint, FiniteFieldError> {
    // Ensure a < p (required for modular arithmetic)
    FiniteField::check_less_than(a, p)?;

    // Compute a^(p-2) mod p using Fermat's Little Theorem
    Ok(a.modpow(&(p - BigUint::from(2u32)), p))
}
```

**How it works:**
1. Validate that `a < p`
2. Compute the exponent: `p - 2`
3. Use modular exponentiation: `a^(p-2) mod p`
4. Return the result, which is `a⁻¹ mod p`

#### Example Calculation

Let's compute `5⁻¹ mod 17`:

```
a⁻¹ = 5^(17-2) mod 17
    = 5^15 mod 17
    = 7 mod 17
```

**Verification:** `5 × 7 = 35 ≡ 1 (mod 17)` ✓

**Note:** We don't actually compute `5^15 = 30,517,578,125` and then take the modulo. The `modpow` function uses **modular exponentiation** (square-and-multiply algorithm), which keeps intermediate results small by applying `mod p` at each step. This is crucial for cryptographic operations with 256-bit numbers.

#### How Modular Exponentiation Works (Square-and-Multiply)

Here's how the algorithm efficiently computes `5^15 mod 17` without ever dealing with the huge number `30,517,578,125`:

**Step 1: Build powers by squaring**
```
5^1 mod 17 = 5
5^2 mod 17 = 25 mod 17 = 8
5^4 mod 17 = 8^2 mod 17 = 64 mod 17 = 13
5^8 mod 17 = 13^2 mod 17 = 169 mod 17 = 16
```

**Step 2: Combine powers to get 5^15**

Since `15 = 8 + 4 + 2 + 1` (binary: `1111`), we have:
```
5^15 = 5^8 × 5^4 × 5^2 × 5^1
     = 16 × 13 × 8 × 5 (mod 17)
```

**Step 3: Multiply step-by-step (applying mod at each step)**
```
16 × 13 = 208 mod 17 = 4
4 × 8 = 32 mod 17 = 15
15 × 5 = 75 mod 17 = 7
```

**Result:** `5^15 mod 17 = 7` ✓

Notice how all intermediate values stay small (under 17), making this computation efficient even for cryptographic-sized exponents!

#### Why This Matters for ECDSA

In **ECDSA (Elliptic Curve Digital Signature Algorithm)**, signature verification requires computing divisions in the finite field. Since division doesn't exist in modular arithmetic, we convert it to multiplication by the inverse:

`a / b (mod n)` becomes `a × b⁻¹ (mod n)`

Specifically, during ECDSA verification, we need to compute:
- `s⁻¹ (mod n)` where `s` is part of the signature and `n` is the curve order

The `inv_mult_prime` function efficiently computes this inverse using Fermat's Little Theorem, enabling the verification process to determine whether a signature is valid. Without the ability to compute multiplicative inverses, ECDSA signature verification would be impossible.

#### Key Takeaway

Fermat's Little Theorem provides an elegant shortcut for computing multiplicative inverses in prime finite fields: simply compute `a^(p-2) mod p`. This operation is fundamental to ECDSA signature verification and many other cryptographic protocols. The `inv_mult_prime` function implements this efficiently using modular exponentiation, making it practical even for the large prime numbers used in modern cryptography.
