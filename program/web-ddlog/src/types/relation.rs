use std::collections::HashMap;
use super::data::PrimitiveValue;



pub struct Relation {
    inner: HashMap<String,PrimitiveValue>
}