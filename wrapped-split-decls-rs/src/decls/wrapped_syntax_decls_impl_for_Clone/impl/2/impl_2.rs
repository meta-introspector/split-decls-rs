use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > Clone for Parse < T > { fn clone (& self) -> Parse < T > { Parse { green : self . green . clone () , errors : self . errors . clone () , _ty : PhantomData , } } }