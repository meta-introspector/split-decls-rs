use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " A piece is a portion of the format string which represents the next part"] # [doc = " to emit. These are emitted as a stream by the `Parser` class."] # [derive (Clone , Debug , PartialEq)] pub enum Piece < 'input > { # [doc = " A literal string which should directly be emitted"] Lit (& 'input str) , # [doc = " This describes that formatting should process the next argument (as"] # [doc = " specified inside) for emission."] NextArgument (Box < Argument < 'input > >) , }
}