
pub trait Convert {
    fn convert(data : f64) -> Self;
}
impl Convert for u8 {
    fn convert(data: f64) -> Self {
        let d = data as u8;
        return d;
    }
}
impl Convert for u16 {
    fn convert(data: f64) -> Self {
        let d = data as u16;
        return d;
    }
}
impl Convert for u32 {
    fn convert(data: f64) -> Self {
        let d = data as u32;
        return d;
    }
}
impl Convert for u64 {
    fn convert(data: f64) -> Self {
        let d = data as u64;
        return d;
    }
}
impl Convert for u128 {
    fn convert(data: f64) -> Self {
        let d = data as u128;
        return d;
    }
}
impl Convert for f64 {
    fn convert(data: f64) -> Self {
        return data;
    }
}
impl Convert for f32 {
    fn convert(data: f64) -> Self {
        return data as f32;
    }
}