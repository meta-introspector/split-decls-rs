use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FromPathError { # [doc = " Converts self into a [`std::io::Error`] with kind"] # [doc = " [`InvalidData`](io::ErrorKind::InvalidData)."] # [doc = ""] # [doc = " Many users of [`FromPathError`] will want to convert it into an [`io::Error`]. This is a"] # [doc = " convenience method to do that."] pub fn into_io_error (self) -> io :: Error { io :: Error :: new (io :: ErrorKind :: InvalidData , self) } }
}