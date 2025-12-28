use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : Decoder , T > Decodable < D > for PhantomData < T > { fn decode (_ : & mut D) -> PhantomData < T > { PhantomData } }
}