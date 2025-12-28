use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait ExtendUnord < T > { # [doc = " Extend this unord collection with the given `UnordItems`."] # [doc = " This method is called `extend_unord` instead of just `extend` so it"] # [doc = " does not conflict with `Extend::extend`. Otherwise there would be many"] # [doc = " places where the two methods would have to be explicitly disambiguated"] # [doc = " via UFCS."] fn extend_unord < I : Iterator < Item = T > > (& mut self , items : UnordItems < T , I >) ; }
}