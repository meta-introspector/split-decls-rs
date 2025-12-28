use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : Encoder , T > Encodable < S > for PhantomData < T > { fn encode (& self , _s : & mut S) { } }
}