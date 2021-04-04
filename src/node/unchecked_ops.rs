use num_traits::{WrappingMul, WrappingSub, WrappingAdd};

pub trait UncheckedOps {
    fn mul(a: Self, b: Self) -> Self;
    fn sub(a: Self, b: Self) -> Self;
    fn add(a: Self, b: Self) -> Self;
}

impl UncheckedOps for u8 {
    fn mul(a: Self, b: Self) -> Self {
        return WrappingMul::wrapping_mul(&a, &b);
    }

    fn sub(a: Self, b: Self) -> Self {
        return WrappingSub::wrapping_sub(&a, &b);
    }

    fn add(a: Self, b: Self) -> Self {
        return WrappingAdd::wrapping_add(&a, &b);
    }
}

impl UncheckedOps for u16 {
    fn mul(a: Self, b: Self) -> Self {
        return WrappingMul::wrapping_mul(&a, &b);
    }
    fn sub(a: Self, b: Self) -> Self {
        return WrappingSub::wrapping_sub(&a, &b);
    }
    fn add(a: Self, b: Self) -> Self {
        return WrappingAdd::wrapping_add(&a, &b);
    }
}

impl UncheckedOps for u32 {
    fn mul(a: Self, b: Self) -> Self {
        return WrappingMul::wrapping_mul(&a, &b);
    }
    fn sub(a: Self, b: Self) -> Self {
        return WrappingSub::wrapping_sub(&a, &b);
    }
    fn add(a: Self, b: Self) -> Self {
        return WrappingAdd::wrapping_add(&a, &b);
    }
}

impl UncheckedOps for u64 {
    fn mul(a: Self, b: Self) -> Self {
        return WrappingMul::wrapping_mul(&a, &b);
    }
    fn sub(a: Self, b: Self) -> Self {
        return WrappingSub::wrapping_sub(&a, &b);
    }
    fn add(a: Self, b: Self) -> Self {
        return WrappingAdd::wrapping_add(&a, &b);
    }
}

impl UncheckedOps for u128 {
    fn mul(a: Self, b: Self) -> Self {
        return WrappingMul::wrapping_mul(&a, &b);
    }
    fn sub(a: Self, b: Self) -> Self {
        return WrappingSub::wrapping_sub(&a, &b);
    }
    fn add(a: Self, b: Self) -> Self {
        return WrappingAdd::wrapping_add(&a, &b);
    }
}

impl UncheckedOps for f64 {
    fn mul(a: Self, b: Self) -> Self {
        return a * b;
    }
    fn sub(a: Self, b: Self) -> Self {
        return a - b;
    }
    fn add(a: Self, b: Self) -> Self {
        return a + b;
    }
}

impl UncheckedOps for f32 {
    fn mul(a: Self, b: Self) -> Self {
        return a * b;
    }
    fn sub(a: Self, b: Self) -> Self {
        return a - b;
    }
    fn add(a: Self, b: Self) -> Self {
        return a + b;
    }
}