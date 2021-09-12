
pub trait Convert {
    fn convert(data : f64) -> Self;
}
impl Convert for i8 {
    fn convert(data: f64) -> Self {
        let d = data as i8;
        return d;
    }
}
impl Convert for i16 {
    fn convert(data: f64) -> Self {
        let d = data as i16;
        return d;
    }
}
impl Convert for i32 {
    fn convert(data: f64) -> Self {
        let d = data as i32;
        return d;
    }
}
impl Convert for i64 {
    fn convert(data: f64) -> Self {
        let d = data as i64;
        return d;
    }
}
impl Convert for i128 {
    fn convert(data: f64) -> Self {
        let d = data as i128;
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