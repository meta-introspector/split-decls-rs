use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : fmt :: Debug > fmt :: Debug for Lock < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . try_lock () { Some (guard) => f . debug_struct ("Lock") . field ("data" , & & * guard) . finish () , None => { struct LockedPlaceholder ; impl fmt :: Debug for LockedPlaceholder { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("<locked>") } } f . debug_struct ("Lock") . field ("data" , & LockedPlaceholder) . finish () } } } }