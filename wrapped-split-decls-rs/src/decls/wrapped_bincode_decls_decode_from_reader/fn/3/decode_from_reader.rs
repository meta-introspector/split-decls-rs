use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Attempt to decode a given type `D` from the given [Reader]."] # [doc = ""] # [doc = " See the [config] module for more information on configurations."] # [doc = ""] # [doc = " [config]: config/index.html"] pub fn decode_from_reader < D : de :: Decode < () > , R : Reader , C : Config > (reader : R , config : C ,) -> Result < D , error :: DecodeError > { let mut decoder = de :: DecoderImpl :: < _ , C , () > :: new (reader , config , ()) ; D :: decode (& mut decoder) }