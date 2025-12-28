use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < L , R , Target > AsMut < [Target] > for Either < L , R > where L : AsMut < [Target] > , R : AsMut < [Target] > , { fn as_mut (& mut self) -> & mut [Target] { for_both ! (self , inner => inner . as_mut ()) } }