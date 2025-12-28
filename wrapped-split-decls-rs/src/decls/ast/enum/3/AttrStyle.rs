use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Distinguishes between `Attribute`s that decorate items and Attributes that"] # [doc = " are contained as statements within items. These two cases need to be"] # [doc = " distinguished for pretty-printing."] # [derive (Clone , PartialEq , Encodable , Decodable , Debug , Copy , HashStable_Generic , Walkable)] pub enum AttrStyle { Outer , Inner , }