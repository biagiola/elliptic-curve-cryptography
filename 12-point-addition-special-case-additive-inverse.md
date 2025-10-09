# Elliptic Curve Cryptography - Course Notes

## Elliptic Curves

### Lesson 12: Point Addition Special Case - Additive Inverse

This lesson covers a critical special case in elliptic curve point addition that must be handled to prevent division-by-zero errors and ensure correct cryptographic operations.

#### The Special Case

When adding two points `P = (x₁, y₁)` and `Q = (x₂, y₂)` on an elliptic curve, there's a special case where:

- `x₁ = x₂` (same x-coordinate)
- `y₁ + y₂ = 0 (mod p)` (y-coordinates are additive inverses)

This means `Q = (x₁, -y₁)` — the points are **reflections of each other across the x-axis**.

#### The Mathematical Reason

In this case, `Q` is the **additive inverse** of `P`, denoted as `-P`. When you add a point to its additive inverse, the result must be the **identity element** (point at infinity):

`P + (-P) = 𝒪`

This is analogous to regular arithmetic where `a + (-a) = 0`.

#### The Geometric Interpretation

When you try to add a point to its reflection:

1. **Draw a line through P and -P**: Since both points have the same x-coordinate but opposite y-coordinates, this line is **vertical** (parallel to the y-axis)

2. **Find the third intersection**: A vertical line doesn't intersect the curve at a third point—it only touches at these two reflection points

3. **Result**: By definition, when there's no third intersection point, the result is the **point at infinity** `𝒪` (the identity element)

#### Why This Check Must Come Early

The code checks this condition **before** computing the slope `s`, because without this check we would encounter a division-by-zero error:

```rust
// Standard slope formula for distinct points:
s = (y₂ - y₁) / (x₂ - x₁)

// But when x₁ = x₂ and y₁ = -y₂:
// - Numerator: y₂ - y₁ = -y₁ - y₁ = -2y₁ ≠ 0
// - Denominator: x₂ - x₁ = 0
// - Result: Division by zero! 💥
```

The check prevents this error by detecting the special case and returning the identity element directly:

```rust
let y1_plus_y2 = FiniteField::add(&y1, &y2, &self.p);
if x1 == x2 && y1_plus_y2 == BigUint::from(0u32) {
    return Point::Identity;
}
```

#### Implementation Details

The check works as follows:

1. **Compute `y₁ + y₂ (mod p)`**: Add the y-coordinates in the finite field
2. **Check if result is zero**: If `y₁ + y₂ ≡ 0 (mod p)`, then `y₂ = -y₁`
3. **Check if x-coordinates match**: If `x₁ = x₂` as well, we have the special case
4. **Return identity**: Both conditions met means `P + Q = 𝒪`

#### Why It's Important for Cryptography

As mentioned in the course, this check is essential for several cryptographic operations:

**1. Computing Scalar Multiplication**

When computing `kP = P + P + ... + P` (k times), you'll eventually reach a point where:

`nP = 𝒪` (where `n` is the order of point `P`)

This check ensures that case is handled correctly.

**2. Finding the Order of a Point**

To find how many times you need to add a point to itself to get the identity, you need to correctly detect when you've reached `𝒪`.

**3. Verifying Group Properties**

Every element in a group must have an inverse. This check ensures that:
- `P + (-P) = 𝒪` (inverse property)
- `P + 𝒪 = P` (identity property)

**4. Implementing Cryptographic Protocols**

Protocols like ECDSA (Elliptic Curve Digital Signature Algorithm) rely on correct point arithmetic. Missing this case would cause crashes or incorrect results.

#### Example

Using the elliptic curve from the course:

```rust
// Point P = (16, 5)
// Point Q = (16, 1)  where 5 + 1 ≡ 0 (mod p)

// Q is the reflection of P across the x-axis
// Therefore: P + Q = 𝒪 (point at infinity)
```

The test verifies this:
```rust
#[test]
fn point_addition_opposites_results_in_identity() {
    // Test that adding a point to its reflection gives identity
    let result = curve.add(P, Q);
    assert_eq!(result, Point::Identity);
}
```

#### Performance Trade-off

The transcription notes: **"it reduces a bit the performance since we have this check at the beginning, but we have to do it."**

This is a necessary correctness check. The performance impact is minimal (just two comparisons), but without it, the implementation would crash on mathematically valid operations. This is a classic example of a **correctness-first** design decision in cryptographic implementations.

#### Key Takeaway

This special case check detects when you're adding a point to its own reflection (additive inverse). Geometrically, this corresponds to a vertical line that doesn't intersect the curve at a third point, so the result must be the identity element (point at infinity). Without this check, the slope calculation would attempt division by zero, causing the implementation to fail. This check is essential for correct cryptographic operations and must be performed before computing the slope.
