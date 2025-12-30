use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: borrow_decode_from_slice_with_context");
# [doc = " Attempt to decode a given type `D` from the given slice with `Context`. Returns the decoded output and the amount of bytes read."] # [doc = ""] # [doc = " See the [config] module for more information on configurations."] # [doc = ""] # [doc = " [config]: config/index.html"] pub fn borrow_decode_from_slice_with_context < 'a , Context , D : de :: BorrowDecode < 'a , Context > , C : Config , > (src : & 'a [u8] , config : C , context : Context ,) -> Result < (D , usize) , error :: DecodeError > { let reader = de :: read :: SliceReader :: new (src) ; let mut decoder = de :: DecoderImpl :: < _ , C , Context > :: new (reader , config , context) ; let result = D :: borrow_decode (& mut decoder) ? ; let bytes_read = src . len () - decoder . reader () . slice . len () ; Ok ((result , bytes_read)) }
}