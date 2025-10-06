# Elliptic Curve Cryptography - Course Notes

## Introduction

### Lesson 4: Theorem - Generating Groups Under Multiplication

There's a powerful theorem that allows us to systematically generate groups using modular multiplication. This theorem is particularly useful in cryptography because it gives us a reliable way to construct finite abelian groups with specific properties.

#### The Theorem

Given a set of integers under **modular multiplication modulo n**, where the set contains elements `{0, 1, 2, ..., n-1}` that satisfy the condition that the **greatest common divisor (GCD)** between each element `i` and `n` equals 1 (i.e., `gcd(i, n) = 1`), these elements form an **abelian group** under multiplication modulo n. The identity element in this group is `1`.

In mathematical notation: If we take all integers from 0 to n-1 that are **coprime** to n (meaning they share no common factors with n except 1), they form an abelian group under multiplication modulo n.

#### Example: (ℤ₉*, ×) where ℤ₉* = {i : gcd(i, 9) = 1}

Let's verify this theorem with `n = 9`. We need to find all numbers from 0 to 8 where `gcd(i, 9) = 1`:

- `gcd(0, 9) = 9` ✗
- `gcd(1, 9) = 1` ✓
- `gcd(2, 9) = 1` ✓
- `gcd(3, 9) = 3` ✗
- `gcd(4, 9) = 1` ✓
- `gcd(5, 9) = 1` ✓
- `gcd(6, 9) = 3` ✗
- `gcd(7, 9) = 1` ✓
- `gcd(8, 9) = 1` ✓

So our set is `{1, 2, 4, 5, 7, 8}`. Notice that 0, 3, and 6 are excluded because they share common factors with 9.

#### Verification Using a Multiplication Table

To prove this is a group under modular multiplication (mod 9), we can construct a multiplication table. Each cell shows the result of `(row × column) mod 9`:

```
×  | 1  2  4  5  7  8
---|------------------
1  | 1  2  4  5  7  8
2  | 2  4  8  1  5  7
4  | 4  8  7  2  1  5
5  | 5  1  2  7  8  4
7  | 7  5  1  8  4  2
8  | 8  7  5  4  2  1
```

Let's verify the group properties:

- **Closure**: ✓ Every entry in the table is an element from our set `{1, 2, 4, 5, 7, 8}`. No matter which two elements we multiply, the result (after applying mod 9) stays within the set.

- **Associativity**: ✓ Multiplication is associative, and this property is preserved under modular arithmetic.

- **Identity Element**: ✓ The number `1` serves as the identity. Looking at the first row and first column, we can see that `1 × a = a` and `a × 1 = a` for all elements.

- **Inverses**: ✓ Every element has an inverse. We can verify this by checking that each row contains the identity element `1` exactly once. When we find `1` in a row, the column header gives us the inverse:
  - `1 × 1 = 1` (1 is its own inverse)
  - `2 × 5 = 10 mod 9 = 1` (2 and 5 are inverses)
  - `4 × 7 = 28 mod 9 = 1` (4 and 7 are inverses)
  - `5 × 2 = 10 mod 9 = 1` (5 and 2 are inverses)
  - `7 × 4 = 28 mod 9 = 1` (7 and 4 are inverses)
  - `8 × 8 = 64 mod 9 = 1` (8 is its own inverse)

- **Commutativity**: ✓ The table is symmetric along the diagonal, meaning `a × b = b × a` for all elements.

Since all properties are satisfied, this is indeed an **abelian group**.

#### Corollary: When n is Prime

There's an important special case of this theorem: **What happens when n is a prime number?**

When `n` is prime, something remarkable occurs. The greatest common divisor between any number `i` (where `1 ≤ i < n`) and the prime number `n` is **always equal to 1**. This is because all numbers less than a prime are **relatively prime** to that prime—they share no common factors except 1.

**Example: (ℤ₇*, ×) where n = 7 (prime)**

If we take the integers under modular multiplication modulo 7, we get the **complete set** of non-zero elements:

`{1, 2, 3, 4, 5, 6}`

Notice that we include **all** numbers from 1 to 6. None are excluded because:
- `gcd(1, 7) = 1` ✓
- `gcd(2, 7) = 1` ✓
- `gcd(3, 7) = 1` ✓
- `gcd(4, 7) = 1` ✓
- `gcd(5, 7) = 1` ✓
- `gcd(6, 7) = 1` ✓

This means that when working with a prime modulus, we automatically get a group containing all non-zero elements less than the prime. This property makes prime numbers especially valuable in cryptography, as they give us the maximum number of elements to work with in our group.

#### Key Takeaway

This theorem provides a systematic method for generating **finite abelian groups** using modular multiplication. By selecting elements that are coprime to the modulus n, we're guaranteed to have a valid group structure. When `n` is prime, this becomes even simpler—we get all non-zero elements automatically. This construction is fundamental in cryptography, particularly in systems that rely on the difficulty of certain mathematical problems in these groups (such as the discrete logarithm problem, which we'll encounter later in elliptic curve cryptography).
