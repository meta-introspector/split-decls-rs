use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Write for CurlSubtransport { fn write (& mut self , data : & [u8]) -> io :: Result < usize > { if self . reader . is_none () { self . execute (data) ? ; } Ok (data . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }