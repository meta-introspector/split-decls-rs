use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T , I : Iterator < Item = T > > From < UnordItems < T , I > > for UnordBag < T > { fn from (value : UnordItems < T , I >) -> Self { UnordBag { inner : Vec :: from_iter (value . 0) } } }