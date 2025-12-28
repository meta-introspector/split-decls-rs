use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Default hash builder, matches hashbrown's default hasher."] # [doc = ""] # [doc = " See [`DefaultHasher`] for more details."] # [derive (Clone , Default , Debug)] pub struct DefaultHashBuilder (hashbrown :: DefaultHashBuilder) ;
}