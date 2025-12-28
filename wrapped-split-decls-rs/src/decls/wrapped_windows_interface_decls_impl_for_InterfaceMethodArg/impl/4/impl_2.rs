use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl InterfaceMethodArg { fn borrow_type (& self) -> Option < (syn :: Type , String) > { if let syn :: Type :: Path (path) = & * self . ty { if let Some (segment) = path . path . segments . last () { let ident = segment . ident . to_string () ; if matches ! (ident . as_str () , "Ref" | "OutRef") { if let syn :: PathArguments :: AngleBracketed (args) = & segment . arguments { if args . args . len () == 1 { if let Some (syn :: GenericArgument :: Type (ty)) = args . args . first () { return Some ((ty . clone () , ident)) ; } } } } } } None } }
}