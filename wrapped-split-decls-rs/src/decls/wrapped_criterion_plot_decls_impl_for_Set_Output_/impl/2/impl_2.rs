use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Set < Output > for Figure { # [doc = " Changes the output file"] # [doc = ""] # [doc = " **Note** The default output file is `output.plot`"] fn set (& mut self , output : Output) -> & mut Figure { self . output = output . 0 ; self } }