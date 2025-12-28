use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Register an expansion which has been decoded from the on-disk-cache for the local crate."] pub fn register_local_expn_id (data : ExpnData , hash : ExpnHash) -> ExpnId { HygieneData :: with (| hygiene_data | { let expn_id = hygiene_data . local_expn_data . next_index () ; hygiene_data . local_expn_data . push (Some (data)) ; let _eid = hygiene_data . local_expn_hashes . push (hash) ; debug_assert_eq ! (expn_id , _eid) ; let expn_id = expn_id . to_expn_id () ; let _old_id = hygiene_data . expn_hash_to_expn_id . insert (hash , expn_id) ; debug_assert ! (_old_id . is_none ()) ; expn_id }) }
}