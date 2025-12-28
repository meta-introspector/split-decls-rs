use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Register an expansion which has been decoded from the metadata of a foreign crate."] pub fn register_expn_id (krate : CrateNum , local_id : ExpnIndex , data : ExpnData , hash : ExpnHash ,) -> ExpnId { debug_assert ! (data . parent == ExpnId :: root () || krate == data . parent . krate) ; let expn_id = ExpnId { krate , local_id } ; HygieneData :: with (| hygiene_data | { let _old_data = hygiene_data . foreign_expn_data . insert (expn_id , data) ; let _old_hash = hygiene_data . foreign_expn_hashes . insert (expn_id , hash) ; debug_assert ! (_old_hash . is_none () || _old_hash == Some (hash)) ; let _old_id = hygiene_data . expn_hash_to_expn_id . insert (hash , expn_id) ; debug_assert ! (_old_id . is_none () || _old_id == Some (expn_id)) ; }) ; expn_id }
}