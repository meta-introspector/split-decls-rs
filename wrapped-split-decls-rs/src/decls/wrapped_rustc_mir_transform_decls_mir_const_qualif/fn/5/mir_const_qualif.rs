use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn mir_const_qualif (tcx : TyCtxt < '_ > , def : LocalDefId) -> ConstQualifs { let body = & tcx . mir_built (def) . borrow () ; let ccx = check_consts :: ConstCx :: new (tcx , body) ; match ccx . const_kind { Some (ConstContext :: Const { .. } | ConstContext :: Static (_) | ConstContext :: ConstFn) => { } None => { span_bug ! (tcx . def_span (def) , "`mir_const_qualif` should only be called on const fns and const items") } } if body . return_ty () . references_error () { tcx . dcx () . span_delayed_bug (body . span , "mir_const_qualif: MIR had errors") ; return Default :: default () ; } let mut validator = check_consts :: check :: Checker :: new (& ccx) ; validator . check_body () ; validator . qualifs_in_return_place () }
}