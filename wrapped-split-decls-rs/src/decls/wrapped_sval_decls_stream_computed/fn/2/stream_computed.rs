use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = "\nStream a value through a stream with an arbitrarily short lifetime.\n"] pub fn stream_computed < 'sval > (stream : & mut (impl Stream < 'sval > + ? Sized) , value : impl Value ,) -> Result { stream . value_computed (& value) }
}