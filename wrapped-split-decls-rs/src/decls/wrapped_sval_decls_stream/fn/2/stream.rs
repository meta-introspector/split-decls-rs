use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = "\nStream a value through a stream.\n"] pub fn stream < 'sval > (stream : & mut (impl Stream < 'sval > + ? Sized) , value : & 'sval (impl Value + ? Sized) ,) -> Result { stream . value (value) }