# Elliptic Curve Cryptography - Course Notes

## Elliptic Curves

### Lesson 11: Introduction to Elliptic Curves

We've finally arrived at the heart of elliptic curve cryptography. Until now, we've been working with groups based on modular arithmetic with simple numbers. Now we're going to define a completely different kind of group—one where the elements are **points** on a curve, and the group operation is a geometric addition rule.

#### The Elliptic Curve Equation

An **elliptic curve** is defined by the equation:

`y² = x³ + ax + b (mod p)`

Where:
- `a` and `b` are coefficients that define the specific curve
- `p` is a large prime number (typically `p > 3`)
- All arithmetic is done modulo `p`

**Non-singularity constraint:** The coefficients must satisfy `4a³ + 27b² ≠ 0` to ensure the curve has no cusps or self-intersections.

#### Visualizing Elliptic Curves

**In the real number field** (without modular arithmetic), an elliptic curve looks like a smooth, continuous curve that's symmetric about the x-axis. This is the visualization we typically see in diagrams.

**In the finite field** (with modulo `p`), the curve looks completely different—it appears as a seemingly random scatter of discrete points distributed across the coordinate plane. The smooth curve disappears, and we're left with only the points `(x, y)` that satisfy the equation modulo `p`. This discrete nature is what makes the curve useful for cryptography, but it also makes it impossible to plot as nicely as the continuous version.

#### Elements of the Group

Unlike the groups we've studied before (where elements were single numbers), the elements of an elliptic curve group are **points**:

`P = (x, y)`

Where both `x` and `y` satisfy the elliptic curve equation modulo `p`.

The group operation is **point addition**, which we'll denote as `P + Q` (not multiplication). This operation combines two points to produce a third point that also lies on the curve.

#### The Geometric Addition Rule

The beauty of elliptic curves is that point addition has an elegant geometric interpretation. Here's how to add two points `P` and `Q`:

**Step 1: Draw a line through P and Q**

Draw a straight line that passes through both points `P` and `Q`.

**Step 2: Find the third intersection point**

Due to the cubic nature of the equation (`x³` term), this line will intersect the curve at exactly **one more point**. Call this point `R'`.

**Step 3: Reflect across the x-axis**

The result of `P + Q` is the reflection of `R'` across the x-axis. This gives us point `R`.

Mathematically: `P + Q = R`

**Why reflection?** The elliptic curve is symmetric about the x-axis (because of the `y²` term). If `(x, y)` is on the curve, then `(x, -y)` is also on the curve. The reflection step ensures that the addition operation has the right algebraic properties to form a group.

#### Special Case 1: The Point at Infinity (Identity Element)

What happens when we try to add two points that are vertically aligned? For example:

`P = (x, y)` and `Q = (x, -y)`

These points lie on a vertical line. A vertical line doesn't intersect the curve at a third point—it only touches at these two points. To handle this case, we define a special **point at infinity**, denoted `𝒪` (or sometimes `∞`), which serves as the **identity element** of the group.

**Properties of the point at infinity:**
- `P + 𝒪 = P` for any point `P`
- `P + (-P) = 𝒪` (where `-P` is the reflection of `P` across the x-axis)
- If `P = (x, y)`, then `-P = (x, -y)`

The point at infinity doesn't have a physical location on the curve—it's a mathematical abstraction that makes the group structure work. Think of it as the "zero" of point addition.

#### Special Case 2: Point Doubling (P + P)

What if we want to add a point to itself? `P + P = 2P`

In this case, we can't draw a line through two distinct points. Instead, we:

**Step 1: Draw the tangent line at P**

Find the tangent line to the curve at point `P`.

**Step 2: Find where the tangent intersects the curve**

This tangent line will intersect the curve at exactly one other point, `R'`.

**Step 3: Reflect across the x-axis**

The result `2P = R` is the reflection of `R'`.

Point doubling is crucial because it allows us to compute scalar multiplication efficiently: `kP = P + P + ... + P` (`k` times).

#### The Point Addition Formulas

While the geometric interpretation is intuitive, we need algebraic formulas to actually compute point addition. Given two points:

`P = (x₁, y₁)` and `Q = (x₂, y₂)`

The result `R = P + Q = (x₃, y₃)` is computed as:

```
x₃ = s² - x₁ - x₂ (mod p)
y₃ = s(x₁ - x₃) - y₁ (mod p)
```

Where the slope `s` depends on whether we're adding distinct points or doubling:

**Case 1: P ≠ Q (distinct points)**
```
s = (y₂ - y₁) / (x₂ - x₁) (mod p)
```

**Case 2: P = Q (point doubling)**
```
s = (3x₁² + a) / (2y₁) (mod p)
```

**Important:** Division in modular arithmetic means multiplying by the modular inverse. For example, `a / b (mod p)` is computed as `a × b⁻¹ (mod p)`, where `b⁻¹` is the multiplicative inverse of `b` modulo `p`.

#### Finite Fields: The Foundation

The operations in the formulas above (addition, multiplication, division) are performed in a **finite field** `𝔽ₚ`, which is the set of integers modulo `p` with both addition and multiplication operations.

A **finite field** is like a group, but with two operations:
- **Addition** with identity `0` and inverses `-a`
- **Multiplication** with identity `1` and inverses `a⁻¹` (for `a ≠ 0`)

Both operations satisfy the group properties (closure, associativity, identity, inverses), and they interact nicely through the distributive law: `a(b + c) = ab + ac`.

When we work with elliptic curves over `𝔽ₚ`, all the coordinate arithmetic happens in this finite field.

#### Why Two Cases Matter for Implementation

When implementing elliptic curve operations, you must handle both cases:

1. **Distinct point addition** (`P ≠ Q`): Uses the slope formula `(y₂ - y₁) / (x₂ - x₁)`
2. **Point doubling** (`P = Q`): Uses the tangent slope formula `(3x₁² + a) / (2y₁)`

These two cases allow us to compute any scalar multiplication `kP` efficiently, which is essential for implementing the discrete logarithm problem on elliptic curves. We'll see in upcoming lessons how this enables cryptographic protocols.

#### Key Takeaway

Elliptic curves provide a new kind of group where:
- **Elements are points** `(x, y)` on the curve, not single numbers
- **The operation is point addition**, defined geometrically by line intersections
- **The identity is the point at infinity** `𝒪`
- **Inverses are reflections** across the x-axis: `-P = (x, -y)`
- **All arithmetic is done modulo p** in a finite field

The geometric addition rule translates into algebraic formulas that we can implement in code. This structure allows us to define the discrete logarithm problem in a new setting—one that's believed to be even harder to solve than the traditional DLP, making elliptic curves extremely powerful for cryptography.
