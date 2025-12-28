use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < E : rustc_serialize :: Encoder > Encodable < E > for ErrorGuaranteed { # [inline] fn encode (& self , _e : & mut E) { panic ! ("should never serialize an `ErrorGuaranteed`, as we do not write metadata or \
            incremental caches in case errors occurred") } }
}