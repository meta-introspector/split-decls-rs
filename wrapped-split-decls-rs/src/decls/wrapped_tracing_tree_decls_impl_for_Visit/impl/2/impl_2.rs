use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Visit for Data { fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { self . kvs . push ((field . name () , format ! ("{:?}" , value))) } }