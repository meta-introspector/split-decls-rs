use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Indicates whether a generated statement needs a trailing semicolon."] # [doc = ""] # [doc = " This enum is used in the context of macro expansion, particularly when generating"] # [doc = " statements, to manage the presence or absence of a trailing semicolon."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum AddSemicolon { # [doc = " The generated statement requires a semicolon."] Yes , # [doc = " The generated statement does not require a semicolon."] No , }
}