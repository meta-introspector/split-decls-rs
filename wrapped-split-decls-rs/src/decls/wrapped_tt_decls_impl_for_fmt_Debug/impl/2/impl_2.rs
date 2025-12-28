use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : fmt :: Debug + Copy > fmt :: Debug for TopSubtree < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . view () , f) } }