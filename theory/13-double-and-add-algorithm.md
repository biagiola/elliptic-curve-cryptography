# Double-and-Add Algorithm

## Overview

The **Double-and-Add Algorithm** is an efficient method for computing scalar multiplication of points on elliptic curves. This algorithm significantly reduces computational complexity when multiplying a point by a large scalar value.

## Table of Contents

- [Problem Statement](#problem-statement)
- [Algorithm Description](#algorithm-description)
- [Mathematical Foundation](#mathematical-foundation)
- [Implementation Steps](#implementation-steps)
- [Worked Example](#worked-example)
- [Complexity Analysis](#complexity-analysis)
- [Use Cases](#use-cases)

---

## Problem Statement

### Naive Approach

Computing `d · A` where:
- `d` is a very large scalar (integer)
- `A` is a point on an elliptic curve

The naive approach would be:
```
d · A = A + A + A + ... + A  (d times)
```

**Problem**: This is extremely inefficient for large values of `d`.

### Solution

The Double-and-Add algorithm reduces the number of operations from `O(d)` to `O(log d)` by leveraging the binary representation of the scalar.

---

## Algorithm Description

### Core Concept

1. **Convert** the scalar `d` to binary representation
2. **Iterate** through each bit from most significant to least significant
3. **Double** the temporary result on each iteration
4. **Add** the point `A` when the current bit is `1`

### Pseudocode

```
Input: Scalar d (decimal), Point A
Output: Point T = d · A

1. Convert d to binary: d = (b_{t-1}, b_{t-2}, ..., b_1, b_0)_2
2. T ← A
3. For i from (t-1) down to 0:
     a. T ← 2T  (point doubling)
     b. If b_i = 1:
          T ← T + A  (point addition)
4. Return T
```

---

## Mathematical Foundation

### Binary Representation

Any decimal number can be represented as a sum of powers of 2:

```
d = Σ(b_i · 2^i) where b_i ∈ {0, 1}
```

**Example**: 
```
236₁₀ = 11101100₂
      = 1·2⁷ + 1·2⁶ + 1·2⁵ + 0·2⁴ + 1·2³ + 1·2² + 0·2¹ + 0·2⁰
      = 128 + 64 + 32 + 8 + 4
```

### Point Operations

- **Point Doubling**: `2P = P + P`
- **Point Addition**: `P + Q` (where P ≠ Q)

---

## Implementation Steps

### Step-by-Step Process

1. **Initialize**: Set `T = A`
2. **Process each bit** (from MSB to LSB):
   - **Always double**: `T = 2T`
   - **Conditionally add**: If bit is `1`, then `T = T + A`

### Visual Representation

```
Binary: 1  1  1  0  1  1  0  0
        ↓  ↓  ↓  ↓  ↓  ↓  ↓  ↓
Step:   0  1  2  3  4  5  6  7
```

---

## Worked Example

### Problem: Calculate 236 · A

**Binary representation**: `236₁₀ = 11101100₂`

| Step | Bit | Operation | Result |
|------|-----|-----------|--------|
| 0 (Init) | 1 | T = A | T = A |
| 1 | 1 | T = 2T + A | T = 2A + A = 3A |
| 2 | 1 | T = 2T + A | T = 6A + A = 7A |
| 3 | 0 | T = 2T | T = 14A |
| 4 | 1 | T = 2T + A | T = 28A + A = 29A |
| 5 | 1 | T = 2T + A | T = 58A + A = 59A |
| 6 | 0 | T = 2T | T = 118A |
| 7 | 0 | T = 2T | T = 236A |

**Result**: `236 · A` computed in **7 steps** instead of 236 additions!

### Detailed Calculation

```
Step 0: T = A
Step 1: T = 2A + A = 3A          (bit = 1)
Step 2: T = 6A + A = 7A          (bit = 1)
Step 3: T = 14A                  (bit = 0)
Step 4: T = 28A + A = 29A        (bit = 1)
Step 5: T = 58A + A = 59A        (bit = 1)
Step 6: T = 118A                 (bit = 0)
Step 7: T = 236A                 (bit = 0)
```

---

## Complexity Analysis

### Time Complexity

- **Naive approach**: `O(d)` - requires `d` point additions
- **Double-and-Add**: `O(log₂ d)` - requires at most `⌈log₂ d⌉` doublings and additions

### Space Complexity

- `O(1)` - only requires storage for temporary point `T`

### Performance Comparison

| Scalar (d) | Naive Operations | Double-and-Add Operations | Improvement |
|------------|------------------|---------------------------|-------------|
| 236 | 236 | 7 | 33.7x faster |
| 1,000 | 1,000 | 10 | 100x faster |
| 1,000,000 | 1,000,000 | 20 | 50,000x faster |
| 2²⁵⁶ | ~10⁷⁷ | 256 | Astronomical |

---

## Use Cases

### Elliptic Curve Cryptography

The Double-and-Add algorithm is fundamental to:

- **ECDSA** (Elliptic Curve Digital Signature Algorithm)
- **ECDH** (Elliptic Curve Diffie-Hellman)
- **Key generation** in ECC systems
- **Point multiplication** in cryptographic protocols

### Why It Matters

In cryptography, scalars are typically 256-bit numbers (or larger). Without this optimization:
- Operations would be computationally infeasible
- Cryptographic systems would be impractical
- Real-time encryption/decryption would be impossible

---

## Implementation Considerations

### Security Notes

⚠️ **Side-Channel Attacks**: The basic algorithm is vulnerable to timing attacks because:
- Addition operations only occur when bit = 1
- Attackers can measure timing differences to deduce the scalar

**Mitigation**: Use constant-time implementations (e.g., Montgomery Ladder)

### Optimization Techniques

1. **Window Methods**: Process multiple bits at once
2. **Non-Adjacent Form (NAF)**: Reduce the number of additions
3. **Precomputation**: Store multiples of the base point

---

## References

- **Elliptic Curve Cryptography**: Standards and implementations
- **Binary Exponentiation**: Similar algorithm for modular exponentiation
- **Cryptographic Protocols**: ECDSA, ECDH specifications

---

## Next Steps

This algorithm will be implemented as the final building block of the elliptic curve cryptography library, enabling efficient scalar multiplication operations essential for cryptographic applications.

---

**Document Version**: 1.0  
**Last Updated**: 2025-10-10  
**Status**: Ready for Implementation
