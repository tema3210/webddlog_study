#[derive(Clone, PartialEq)]
pub enum PrimitiveValue {
    String(arcstr::ArcStr),
    Boolean(bool),
    Integer(i64),
    Float(f32),
    Double(f64),
}