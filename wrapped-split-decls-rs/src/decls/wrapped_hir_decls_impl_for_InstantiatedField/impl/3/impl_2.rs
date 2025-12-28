use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'db > InstantiatedField < 'db > { # [doc = " Returns the type as in the signature of the struct."] pub fn ty (& self , db : & 'db dyn HirDatabase) -> TypeNs < 'db > { let krate = self . inner . krate (db) ; let interner = DbInterner :: new_with (db , Some (krate . base ()) , None) ; let var_id = self . inner . parent . into () ; let field = db . field_types (var_id) [self . inner . id] ; let ty = field . instantiate (interner , self . args) ; TypeNs :: new (db , var_id , ty) } }