# Elliptic Curve Cryptography - Course Notes

## Introduction

### Lesson 8: A Little Exercise

Let's apply the theorems we've learned by working through a concrete example with a group that has prime order. This exercise will demonstrate the powerful property that when a group's cardinality is prime, almost every element is a generator.

#### The Exercise: Analyzing (ℤ₅*, ×)

Consider the group `ℤ₅* = {1, 2, 3, 4}` under multiplication modulo 5. Since 5 is prime, this group contains all non-zero elements less than 5.

**Group properties:**
- Set: `{1, 2, 3, 4}`
- Operation: Multiplication modulo 5
- Cardinality: `|ℤ₅*| = 4`
- Identity: `1`

#### Step 1: Computing the Order of Each Element

Let's find the order of each element by computing successive powers until we reach the identity:

**Element 1:**
- `ord(1) = 1` (since `1¹ = 1`)

**Element 2:**
- `2¹ = 2`
- `2² = 4 mod 5`
- `2³ = 8 = 3 mod 5`
- `2⁴ = 6 = 1 mod 5` ✓
- `ord(2) = 4`

**Element 3:**
- `3¹ = 3`
- `3² = 9 = 4 mod 5`
- `3³ = 12 = 2 mod 5`
- `3⁴ = 6 = 1 mod 5` ✓
- `ord(3) = 4`

**Element 4:**
- `4¹ = 4`
- `4² = 16 = 1 mod 5` ✓
- `ord(4) = 2`

**Summary table:**

| Element | Order | Divides |G|=4? | Generator? |
|---------|-------|----------------|------------|
| 1       | 1     | ✓              | No (identity) |
| 2       | 4     | ✓              | **Yes** |
| 3       | 4     | ✓              | **Yes** |
| 4       | 2     | ✓              | No |

#### Step 2: Verifying Theorem 2

Notice that all orders (1, 2, 4) divide the group cardinality 4. This confirms **Theorem 2**: `ord(a)` divides `|G|` for every element. ✓

#### Step 3: Identifying Generators

The generators are the elements whose order equals the group cardinality:
- `ord(2) = 4 = |ℤ₅*|` → **2 is a generator**
- `ord(3) = 4 = |ℤ₅*|` → **3 is a generator**

So the set of generators is: **{2, 3}**

#### Step 4: Counting Generators with Theorem 3

According to **Theorem 3**, the number of generators should equal `φ(|G|) = φ(4)`.

Let's compute `φ(4)`:
- Numbers relatively prime to 4 (where `gcd(i, 4) = 1` for `1 ≤ i ≤ 4`): `{1, 3}`
- `φ(4) = 2`

We found exactly 2 generators: {2, 3}. The theorem holds! ✓

#### Step 5: Python Verification

The image shows Python code that verifies these calculations:

```python
p = 5
# Computing powers of each element
[i**t for i in range(1,p)]  # Shows all powers
[i**t % p for i in range(1,p)]  # Shows powers modulo 5
```

This programmatic approach helps verify our manual calculations and can be extended to larger groups.

#### Key Observations

1. **All orders divide the group order**: Every element's order (1, 2, 4) is a divisor of 4, confirming Lagrange's theorem.

2. **Two generators exist**: Elements 2 and 3 both generate the entire group, which matches `φ(4) = 2`.

3. **Most elements are generators**: Since `|ℤ₅*| = 4` is not prime, not all non-identity elements are generators (element 4 is not a generator). However, if we had worked with a group of prime order, we would see that all non-identity elements are generators.

4. **Verification with GCD**: The generators {2, 3} correspond to the numbers relatively prime to 4, which are {1, 3}. Note that element 2 maps to the first position and element 3 maps to the second position in the generator count.

#### Key Takeaway

This exercise demonstrates how the theorems work in practice. By computing element orders, we can identify generators and verify that the number of generators matches Euler's totient function. This systematic approach is essential for selecting appropriate cryptographic parameters in real-world systems.
