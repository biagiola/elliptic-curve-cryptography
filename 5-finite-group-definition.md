# Elliptic Curve Cryptography - Course Notes

## Introduction

### Lesson 5: Finite Group Definition

As we move deeper into cryptography, it's essential to clarify that we'll be working almost exclusively with **finite groups**. These groups are not just convenient—they're fundamental to making cryptography practical and secure.

#### Why Finite Groups Matter in Cryptography

Finite groups are the most important type of groups in cryptography for two key reasons:

1. **Practical constraints**: We have limited computational resources in machines. Working with infinite sets would be impossible to implement in real-world systems.

2. **Mathematical properties**: Only finite groups can produce the expected cryptographic results through the mathematical theorems and operations that have been developed for secure communication.

#### Definition of a Finite Group

A group `(G, ∘)` is **finite** if it has a **finite number of elements**. While this definition is straightforward, it leads to an important concept.

**Cardinality (or Order) of a Group**

The **cardinality** or **order** of a group, denoted as `|G|`, is the number of elements the group contains. This is a fundamental property that tells us the "size" of our group.

#### Example 1: Cardinality of (ℤₙ, +)

Consider the integers under modular addition modulo `n`. The set is:

`ℤₙ = {0, 1, 2, ..., n-1}`

The cardinality of this group is simply:

`|ℤₙ| = n`

For instance, if `n = 5`, then `ℤ₅ = {0, 1, 2, 3, 4}` and `|ℤ₅| = 5`.

#### Example 2: Cardinality of (ℤₙ*, ×) - The Euler Phi Function

This example is more complex. Recall the group we defined earlier: the set of integers under modular multiplication modulo `n`, where we only include elements that are **relatively prime** to `n` (i.e., `gcd(i, n) = 1`).

The set is:

`ℤₙ* = {i : 0 ≤ i < n and gcd(i, n) = 1}`

To find the cardinality of this group, we need to count how many numbers between 0 and n-1 are relatively prime to n. Fortunately, there's a mathematical function that does exactly this: **Euler's totient function** (also called the **Phi function**), denoted as `φ(n)`.

The cardinality is:

`|ℤₙ*| = φ(n)`

**Example with n = 9:**

From our earlier lesson, we found that `ℤ₉* = {1, 2, 4, 5, 7, 8}`. Counting these elements:

`|ℤ₉*| = φ(9) = 6`

The Euler totient function `φ(n)` can be computed in different ways depending on the number:
- For **prime numbers** `p`: `φ(p) = p - 1` (very simple!)
- For **composite numbers**: The calculation is more involved and depends on the prime factorization of `n`

While computing `φ(n)` for arbitrary numbers can be challenging, understanding that it represents the count of elements in our multiplicative group is what matters for cryptography.

#### Key Takeaway

Finite groups are the foundation of cryptographic systems because they provide a manageable, well-defined mathematical structure with a specific number of elements. The **cardinality** or **order** of a group tells us its size, which is crucial for understanding the security properties of cryptographic algorithms. For the multiplicative group `ℤₙ*`, Euler's totient function `φ(n)` gives us this cardinality, making it an essential tool in cryptographic analysis.
