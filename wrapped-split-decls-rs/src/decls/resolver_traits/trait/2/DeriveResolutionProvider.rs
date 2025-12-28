use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait DeriveResolutionProvider < D > { fn take_derive_resolutions (& mut self , expn_id : LocalExpnId) -> Option < Vec < D > > ; }
}