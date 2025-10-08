# Elliptic Curve Cryptography - Course Notes

## Introduction

### Lesson 1: Infinite Groups Example

To understand the algebraic foundations of elliptic curve cryptography, we need to first grasp the concept of **groups**. A **group** is a mathematical structure consisting of a **set** and an **operation** that satisfies specific properties. Let's explore this through concrete examples.

#### Example 1: Integers with Addition (ℤ, +)

Consider the set of integers (denoted as **ℤ**) paired with the addition operation. This set includes all integers: `{..., -2, -1, 0, 1, 2, ...}`. When we examine whether this forms a group, we need to verify four key properties:

- **Closure**: When we add any two integers, the result is always another integer, so this property holds.

- **Associativity**: Addition of integers is associative, meaning `(a + b) + c = a + (b + c)` for any integers.

- **Identity Element**: Under addition, zero serves as the identity because adding zero to any number returns that same number.

- **Inverses**: Every integer has an inverse—for any number `n`, its inverse is `-n`, since `n + (-n) = 0`.

Additionally, integer addition is **commutative** (`a + b = b + a`), making this an **abelian group**.

#### Example 2: Integers without Zero with Multiplication (ℤ - {0}, ×)

Now let's examine a different structure: the set of integers excluding zero (**ℤ - {0}**) with multiplication as the operation. This set contains `{..., -2, -1, 1, 2, ...}` (notice zero is removed). Let's check the group properties again:

- **Closure**: ✓ Multiplying any two non-zero integers gives another non-zero integer.

- **Associativity**: ✓ Multiplication is associative: `(a × b) × c = a × (b × c)`.

- **Identity Element**: ✓ The identity element under multiplication is `1`, since any number multiplied by 1 equals itself.

- **Inverses**: ✗ This is where we encounter a problem.
  - For `a = -1`, the inverse is `-1` itself because `(-1) × (-1) = 1`
  - For `a = 1`, the inverse is `1` since `1 × 1 = 1`
  - But for `a = 2`? We would need to find a number that when multiplied by 2 gives us 1. No such integer exists within our set.

Since not every element has an inverse, this structure is **not a group**.

#### Key Takeaway

These examples demonstrate **infinite groups**—sets with an unlimited number of elements. While they're useful for understanding group theory fundamentals, cryptography relies primarily on **finite groups**. Finite sets provide the mathematical structure needed for practical cryptographic algorithms and the algebraic operations we'll be working with throughout this course. Understanding these infinite examples, however, gives us the foundation to better comprehend the finite groups that power elliptic curve cryptography.
