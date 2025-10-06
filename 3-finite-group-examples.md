# Elliptic Curve Cryptography - Course Notes

## Introduction

### Lesson 3: Finite Group Examples

Now that we understand the abstract definition of groups, let's examine concrete examples using finite sets. These finite groups are particularly important for cryptography, as they provide the mathematical foundation for secure cryptographic operations.

#### Example 1: Modular Addition (ℤ₄, +) - A Valid Group

Consider the set `{0, 1, 2, 3}` with the operation of **modular addition modulo 4**. Every time we add two elements, we apply modulo 4 to the result. Let's verify whether this forms a group by checking all the required properties:

- **Closure**: ✓ When we add any two numbers and apply modulo 4, the result always stays within our set. For instance, `3 + 3 = 6`, and `6 mod 4 = 2`, which is in the set. This holds for all combinations.

- **Associativity**: ✓ Addition is associative, and this property is preserved in modular arithmetic. The equation `(a + b) + c = a + (b + c)` holds even when we apply modulo 4.

- **Identity Element**: ✓ The number `0` serves as the identity element. Adding 0 to any number returns that same number: `a + 0 = a` for all elements in the set.

- **Inverses**: ✓ Every element has an inverse that, when added together, produces the identity (0):
  - `0 + 0 = 0` (0 is its own inverse)
  - `1 + 3 = 4 mod 4 = 0` (1 and 3 are inverses of each other)
  - `2 + 2 = 4 mod 4 = 0` (2 is its own inverse)
  - `3 + 1 = 4 mod 4 = 0` (3 and 1 are inverses of each other)

- **Commutativity**: ✓ Addition is commutative: `a + b = b + a` for all elements.

Since all four required properties are satisfied, and commutativity holds, this is an **abelian group**.

#### Example 2: Modular Multiplication (ℤ₄, ×) - Not a Group

Now let's take the same set `{0, 1, 2, 3}` but use **modular multiplication modulo 4** as the operation. Let's check the group properties:

- **Closure**: We need to include 0 in our set because operations like `2 × 2 = 4 mod 4 = 0` produce zero. So closure requires us to keep 0 in the set.

- **Associativity**: ✓ Multiplication is associative under modulo 4, so this property holds.

- **Identity Element**: If we choose `1` as the identity element, we have `a × 1 = a` for all elements, which works.

- **Inverses**: ✗ This is where we encounter a problem. Let's try to find inverses:
  - `1 × 1 = 1` ✓ (1 is its own inverse)
  - For `2`: We need to find an element that satisfies `2 × ? = 1`
    - `2 × 1 = 2 mod 4 = 2` ✗
    - `2 × 2 = 4 mod 4 = 0` ✗
    - `2 × 3 = 6 mod 4 = 2` ✗
  - **No inverse exists for 2!**

Since the element `2` doesn't have an inverse (there's no `2⁻¹` such that `2 × 2⁻¹ = 1`), this structure is **not a group**.

#### Key Insight

These examples demonstrate something crucial: the same set can form a group under one operation but fail to be a group under a different operation. By simply changing from addition to multiplication (both modulo 4), we went from having a valid abelian group to having a structure that isn't a group at all. This shows how the choice of operation fundamentally determines the algebraic properties of the structure, which is essential to understand when designing cryptographic systems.
