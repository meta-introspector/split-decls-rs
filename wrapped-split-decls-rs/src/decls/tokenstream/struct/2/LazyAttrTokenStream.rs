use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A lazy version of [`AttrTokenStream`], which defers creation of an actual"] # [doc = " `AttrTokenStream` until it is needed."] # [derive (Clone)] pub struct LazyAttrTokenStream (Arc < LazyAttrTokenStreamInner >) ;
}