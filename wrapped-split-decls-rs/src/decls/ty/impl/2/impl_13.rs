use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , Ty : fmt :: Display > fmt :: Debug for TyAndLayout < 'a , Ty > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TyAndLayout") . field ("ty" , & format_args ! ("{}" , self . ty)) . field ("layout" , & self . layout) . finish () } }