use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Clone for SourceFile { fn clone (& self) -> Self { Self { name : self . name . clone () , src : self . src . clone () , src_hash : self . src_hash , checksum_hash : self . checksum_hash , external_src : self . external_src . clone () , start_pos : self . start_pos , normalized_source_len : self . normalized_source_len , unnormalized_source_len : self . unnormalized_source_len , lines : self . lines . clone () , multibyte_chars : self . multibyte_chars . clone () , normalized_pos : self . normalized_pos . clone () , stable_id : self . stable_id , cnum : self . cnum , } } }
}