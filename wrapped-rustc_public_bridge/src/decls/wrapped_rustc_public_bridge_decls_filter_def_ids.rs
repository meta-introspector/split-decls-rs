use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Iterate over the definitions of the given crate.
pub fn filter_def_ids<F, T>(tcx: TyCtxt<'_>, krate: CrateNum, mut func: F) -> Vec<T>
where
    F: FnMut(DefId) -> Option<T>,
{
    if krate == LOCAL_CRATE {
        tcx.iter_local_def_id().filter_map(|did| func(did.to_def_id())).collect()
    } else {
        let num_definitions = tcx.num_extern_def_ids(krate);
        (0..num_definitions)
            .filter_map(move |i| {
                let def_id = DefId {
                    krate,
                    index: rustc_span::def_id::DefIndex::from_usize(i),
                };
                func(def_id)
            })
            .collect()
    }
}
