# Elliptic Curve Cryptography - Course Notes

## Introduction

### Lesson 10: The Discrete Logarithm Problem

This is one of the most important concepts in cryptography. The **Discrete Logarithm Problem (DLP)** is the mathematical foundation underlying elliptic curve cryptography, Diffie-Hellman key exchange, and many other cryptographic protocols. Understanding why this problem is hard to solve is key to understanding why these systems are secure.

#### The Discrete Logarithm Problem for (ℤₚ*, ×)

Let's start with the discrete logarithm problem in the context of modular multiplication, which we've been studying.

**Given:**
- A prime `p`
- The group `(ℤₚ*, ×)` with order `|ℤₚ*| = p - 1`
- A generator `α` of the group
- An element `β ∈ ℤₚ*`

**Problem:** Find the integer `x` (where `1 ≤ x ≤ p - 1`) such that:

`αˣ = β (mod p)`

This value `x` is called the **discrete logarithm** of `β` with respect to base `α`.

#### Comparison with Regular Logarithms

In normal arithmetic (not modular), solving `2ˣ = 8` is straightforward:

`x = log₂(8) = 3`

We simply take the logarithm, and we're done. This is **easy** to compute.

However, in modular arithmetic, there's no simple formula like the logarithm function. The discrete logarithm problem is fundamentally different and much harder.

#### Example: DLP in (ℤ₄₇*, ×)

**Problem:** Find `x` such that `5ˣ = 41 (mod 47)`

**Given:**
- Group: `ℤ₄₇*`
- Generator: `α = 5`
- Target: `β = 41`
- Modulus: `p = 47`

**Solution approach:** The only known general method is **brute force**—we try values of `x` one by one:

- `5¹ = 5 mod 47` ✗
- `5² = 25 mod 47` ✗
- `5³ = 125 = 31 mod 47` ✗
- ...continuing...
- `5¹⁵ = 41 mod 47` ✓

After trying multiple values, we find that **x = 15** is the solution.

#### The Asymmetry: Easy Forward, Hard Backward

This problem has a crucial **asymmetric property**:

**Forward direction (easy):** Given `α` and `x`, computing `β = αˣ` is **easy and cheap**. We can use efficient algorithms like fast exponentiation to compute this quickly, even for very large values.

**Backward direction (hard):** Given `α` and `β`, finding `x` such that `αˣ = β` is **very difficult**. The only known general method is brute force, which becomes computationally infeasible for large groups.

This asymmetry is the foundation of cryptographic security. We can easily create a "public key" by computing `αˣ`, but an attacker cannot easily recover our "private key" `x` from the public information.

#### The Generalized Discrete Logarithm Problem

The discrete logarithm problem isn't limited to modular multiplication—it can be defined for **any** group. This generalization is what allows us to use elliptic curves for cryptography.

**Generalized DLP Definition:**

**Given:**
- A finite cyclic group `G` with order `|G|`
- A generator `α` of `G`
- An element `β ∈ G`

**Problem:** Find the integer `x` (where `1 ≤ x ≤ |G|`) such that:

`β = α ∘ α ∘ ... ∘ α` (`x` times)

Or in exponential notation: `β = αˣ`

**Important:** The notation `αˣ` here doesn't necessarily mean numerical exponentiation—it means applying the group operation `x` times. The specific operation depends on the group.

#### Example: DLP in Different Groups

**For (ℤ₇, +) - Addition Group:**

If we have the group of integers modulo 7 under addition, and we want to solve:

`x · α = β (mod 7)`

This means: `α + α + ... + α` (`x` times) `= β (mod 7)`

For instance, if `α = 3` and `β = 5`, we're looking for how many times to add 3 to itself to get 5 modulo 7.

The notation `αˣ` in this context represents repeated addition, not multiplication.

#### Why Generators Matter for Security

Remember the subgroup example from `ℤ₁₃*`? We found that the subgroup `H₂ = {1, 12}` has only 2 elements.

If we mistakenly use `α = 12` (which is **not** a generator of the full group) and try to solve:

`12ˣ = 1 (mod 13)`

There are only 2 possible values for `x`: either `x = 1` or `x = 2`. An attacker has a **50% chance** of guessing correctly on the first try, and will definitely find it on the second try. This is completely insecure!

This is why we **must** use actual generators, not elements that only generate subgroups. A true generator ensures the full group size is used, making brute force attacks computationally infeasible.

#### Security Through Large Groups

The difficulty of the discrete logarithm problem increases with the size of the group. If `|G|` has many digits (e.g., 256 bits or more), the computational power required for brute force becomes astronomical. This is why cryptographic systems:

1. Use very large prime numbers
2. Ensure the element is a true generator
3. Work with groups that have a large prime order

These properties make the discrete logarithm problem practically unsolvable with current technology, providing the security foundation for modern cryptography.

#### Key Takeaway

The **Discrete Logarithm Problem** is the cornerstone of many cryptographic systems. Its key properties are:

- **Easy to compute forward**: `β = αˣ` is fast to calculate
- **Hard to compute backward**: Finding `x` from `β = αˣ` requires brute force
- **Generalizes to any group**: Works with modular multiplication, elliptic curves, and other algebraic structures
- **Security depends on group size**: Larger groups make brute force infeasible
- **Requires true generators**: Using subgroup elements destroys security

Remember this formula: **`β = αˣ`** in a group `G`. This is the foundation we'll build upon when we move to elliptic curve cryptography, where the same principle applies but with a different, more complex group structure.
