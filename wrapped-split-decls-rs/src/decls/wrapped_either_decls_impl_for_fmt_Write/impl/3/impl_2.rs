use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < L , R > fmt :: Write for Either < L , R > where L : fmt :: Write , R : fmt :: Write , { fn write_str (& mut self , s : & str) -> fmt :: Result { for_both ! (self , inner => inner . write_str (s)) } fn write_char (& mut self , c : char) -> fmt :: Result { for_both ! (self , inner => inner . write_char (c)) } fn write_fmt (& mut self , args : fmt :: Arguments < '_ >) -> fmt :: Result { for_both ! (self , inner => inner . write_fmt (args)) } }