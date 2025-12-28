use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Variants of an enum syntax tree node."] # [doc = ""] # [doc = " The keys in the map are the variant names."] # [doc = ""] # [doc = " Variants are unit variants if they hold no data and tuple variants"] # [doc = " otherwise. The Syn syntax tree does not make use of braced variants."] pub type Variants = IndexMap < String , Vec < Type > > ;