use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// This function computes the symbol name for the given `instance` and the
/// given instantiating crate. That is, if you know that instance X is
/// instantiated in crate Y, this is the symbol name this instance would have.
pub fn symbol_name_for_instance_in_crate<'tcx>(
    tcx: TyCtxt<'tcx>,
    instance: Instance<'tcx>,
    instantiating_crate: CrateNum,
) -> String {
    compute_symbol_name(tcx, instance, || instantiating_crate)
}
