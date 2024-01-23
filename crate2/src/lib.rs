use crate1::{Trait1};
use std::marker::PhantomData
;
pub enum Enum1<GenericStruct> where GenericStruct : Trait1 {
    Val1,
    _Marker(PhantomData<GenericStruct>),
}


