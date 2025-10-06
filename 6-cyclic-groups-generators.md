# Elliptic Curve Cryptography - Course Notes

## Introduction

### Lesson 6: Cyclic Groups and Generators

Cyclic groups are a special class of groups that have remarkable properties, making them particularly valuable in cryptography. To understand them, we first need to introduce the concept of the **order of an element**.

#### Order of an Element

The **order of an element** `a` in a group is the smallest positive integer `k` such that when we apply the group operation `k` times, we get the identity element. In multiplicative notation, this means:

`aᵏ = a · a · a · ... · a = 𝕀` (where `a` is repeated `k` times)

We denote the order of element `a` as `ord(a) = k`.

#### Example 1: Element a = 3 in (ℤ₁₁*, ×)

Let's work with the group of integers relatively prime to 11 under multiplication modulo 11. Since 11 is prime, this group is `ℤ₁₁* = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}`, and the identity element is `1`.

Let's take `a = 3` and repeatedly multiply it by itself:

- `3¹ = 3`
- `3² = 3 × 3 = 9 mod 11`
- `3³ = 9 × 3 = 27 = 5 mod 11`
- `3⁴ = 5 × 3 = 15 = 4 mod 11`
- `3⁵ = 4 × 3 = 12 = 1 mod 11` ✓ (we reached the identity!)

We reached the identity element after 5 multiplications, so `ord(3) = 5`.

If we continue:
- `3⁶ = 1 × 3 = 3 mod 11` (the sequence repeats)
- `3⁷ = 3 × 3 = 9 mod 11`
- And so on...

The sequence cycles through `{3, 9, 5, 4, 1}` and then repeats. Notice that we only generated 5 elements, not all 10 elements in the group.

#### What is a Cyclic Group?

A **cyclic group** is a group where there exists at least one element `a` (called a **generator**) that can generate the entire group by repeatedly applying the group operation. Formally:

A group `G` is cyclic if there exists an element `a ∈ G` such that:

`ord(a) = |G|`

In other words, the order of the element equals the cardinality (size) of the group.

#### What is a Generator?

An element `a` is a **generator** of group `G` if every element `b` in the group can be expressed as some power of `a`:

For every `b ∈ G`, there exists some integer `k` such that `aᵏ = b`

This means by raising `a` to different powers, we can produce every single element in the group.

#### Example 2: Element a = 2 in (ℤ₁₁*, ×) - A Generator!

Now let's try `a = 2` in the same group `(ℤ₁₁*, ×)`:

- `2¹ = 2`
- `2² = 4`
- `2³ = 8`
- `2⁴ = 16 = 5 mod 11`
- `2⁵ = 10`
- `2⁶ = 20 = 9 mod 11`
- `2⁷ = 18 = 7 mod 11`
- `2⁸ = 14 = 3 mod 11`
- `2⁹ = 6`
- `2¹⁰ = 12 = 1 mod 11` ✓ (identity reached!)

We had to multiply 10 times to reach the identity, so `ord(2) = 10`.

Notice that the group `ℤ₁₁*` has exactly 10 elements, and `ord(2) = 10 = |ℤ₁₁*|`. This means `2` is a **generator** of the group! By raising 2 to different powers (1 through 10), we generated all elements: `{2, 4, 8, 5, 10, 9, 7, 3, 6, 1}`.

#### Comparison: Generator vs Non-Generator

- **Element 3**: `ord(3) = 5 ≠ |ℤ₁₁*| = 10` → **Not a generator** (only generates a subgroup of 5 elements)
- **Element 2**: `ord(2) = 10 = |ℤ₁₁*| = 10` → **Is a generator** (generates the entire group)

Since `(ℤ₁₁*, ×)` has at least one generator (element 2), it is a **cyclic group**.

#### Why Cyclic Groups Matter in Cryptography

Cyclic groups and generators are fundamental to elliptic curve cryptography and many other cryptographic systems. The ability to generate an entire group from a single element provides the mathematical foundation for key exchange protocols, digital signatures, and encryption schemes. The security of these systems often relies on the difficulty of the **discrete logarithm problem**: given `b = aᵏ`, finding `k` is computationally hard even though computing `aᵏ` is easy.

Understanding generators and cyclic groups is essential as we move toward elliptic curves, where these concepts will be applied in a more complex geometric setting.
