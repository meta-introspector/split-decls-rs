use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Display for CountError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { CountError :: OutOfBounds => f . write_str ("${count} out of bounds") , CountError :: Misplaced => f . write_str ("${count} misplaced") , } } }