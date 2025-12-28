use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl MaybeItemFn { fn as_ref (& self) -> MaybeItemFnRef < '_ , TokenStream > { MaybeItemFnRef { outer_attrs : & self . outer_attrs , inner_attrs : & self . inner_attrs , vis : & self . vis , sig : & self . sig , brace_token : & self . brace_token , block : & self . block , } } }
}