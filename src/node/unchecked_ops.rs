use num_traits::{WrappingMul, WrappingSub, WrappingAdd};

pub trait UncheckedOps {
    fn mul(a: Self, b: Self) -> Self;
    fn sub(a: Self, b: Self) -> Self;
    fn add(a: Self, b: Self) -> Self;
}

impl UncheckedOps for i8 {
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

impl UncheckedOps for i16 {
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

impl UncheckedOps for i32 {
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

impl UncheckedOps for i64 {
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

impl UncheckedOps for i128 {
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