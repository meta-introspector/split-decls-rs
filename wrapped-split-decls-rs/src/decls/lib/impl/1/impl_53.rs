use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FnContext { # [inline] fn new (migrated : bool) -> Self { FnContext { migrated , _marker : PhantomData } } }
}