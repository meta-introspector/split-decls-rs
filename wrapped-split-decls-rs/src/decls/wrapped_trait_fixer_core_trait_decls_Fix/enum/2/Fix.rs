use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug)] pub enum Fix < S , D , ID > { AddDerive { span : S , trait_name : String } , AddCloneImpl { def_id : D } , RemoveImpl { item_id : ID } , }
}