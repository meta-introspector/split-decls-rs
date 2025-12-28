use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > ErrParse < T > { pub fn new (failures : Vec < Failure >) -> Self { ErrParse { failures , _phantom : std :: marker :: PhantomData } } }
}