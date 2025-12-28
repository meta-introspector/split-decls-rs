use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Parse for Keyword { fn parse (input : ParseStream < '_ >) -> Result < Self > { let name = input . parse () ? ; input . parse :: < Token ! [:] > () ? ; let value = input . parse () ? ; Ok (Keyword { name , value }) } }