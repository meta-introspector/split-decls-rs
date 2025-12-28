use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An enum used for controlling the execution of `fold_while`."] # [doc = ""] # [doc = " See [`.fold_while()`](Itertools::fold_while) for more information."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum FoldWhile < T > { # [doc = " Continue folding with this value"] Continue (T) , # [doc = " Fold is complete and will return this value"] Done (T) , }