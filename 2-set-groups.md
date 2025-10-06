# Elliptic Curve Cryptography - Course Notes

## Introduction

### Lesson 2: Sets and Groups

Before diving into the cryptographic applications, we need to establish the foundational mathematical concepts of sets and groups. These structures form the backbone of modern cryptography.

#### What is a Set?

A **set** is simply a collection of elements. Sets can be finite or infinite, and we typically denote them with capital letters like `A`, `B`, `X`, or `Y`. Here are some examples:

**Finite sets:**
- `{1, 2, 3}` - a set containing three numbers
- `{a, b, c}` - a set containing three letters

**Infinite sets:**
- **ℕ** (Natural numbers): `{1, 2, 3, 4, ...}` - continues without end
- **ℤ** (Integers): `{..., -2, -1, 0, 1, 2, ...}` - extends infinitely in both directions (the notation comes from the German word "Zahlen" meaning numbers)

#### What is a Group?

A **group** is a set equipped with an **operation** (which we'll denote with `∘`) that must satisfy four fundamental properties. The operation could be addition, multiplication, or any custom-defined operation between elements in the set.

**The Four Required Properties:**

- **1. Closure**: For every `a` and `b` belonging to set `A`, the result of `a ∘ b` must also belong to `A`. In other words, operating on any two elements in the set always produces another element that's still in the set.

- **2. Associativity**: For every `a`, `b`, and `c` belonging to `A`, the equation `(a ∘ b) ∘ c = a ∘ (b ∘ c)` must hold. This means the order in which we group operations doesn't matter—we get the same result either way.

- **3. Identity Element**: There must exist a special element `𝕀` in `A` such that for every element `a` in `A`, the equation `a ∘ 𝕀 = a` holds. This identity element leaves any element unchanged when operated with it.

- **4. Inverse**: For every element `a` in `A`, there must exist an element `a⁻¹` (also in `A`) such that `a ∘ a⁻¹ = 𝕀`. Every element must have a corresponding inverse that, when operated together, produces the identity element.

**The Optional Fifth Property:**

- **5. Commutativity**: For every `a` and `b` belonging to `A`, the equation `a ∘ b = b ∘ a` must hold. This means the order of the operands doesn't matter.

When a group satisfies this fifth property, it's called an **abelian group**. Throughout this course, we'll primarily work with abelian groups.

#### Why Groups Matter for Cryptography

These definitions might seem abstract, but they're crucial for cryptography. The group structure allows us to define rigorous theorems that make cryptographic methods **secure** and **unpredictable**. The mathematical properties ensure that cryptographic operations behave in well-defined, verifiable ways, which is essential for building systems we can trust.

In the following lessons, we'll explore concrete examples of groups to solidify these concepts and see how they apply to elliptic curve cryptography.
