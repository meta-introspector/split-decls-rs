use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn generic_args_from_tys < 'db > (interner : DbInterner < 'db > , def_id : SolverDefId , args : impl IntoIterator < Item = Ty < 'db > > ,) -> GenericArgs < 'db > { let mut args = args . into_iter () ; GenericArgs :: for_item (interner , def_id , | _ , id , _ | { if matches ! (id , GenericParamId :: TypeParamId (_)) && let Some (arg) = args . next () { arg . into () } else { next_solver :: GenericArg :: error_from_id (interner , id) } } ,) }
}