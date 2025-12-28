use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct DefIdVisitorSkeleton < 'v , 'tcx , V : ? Sized > { def_id_visitor : & 'v mut V , visited_tys : FxHashSet < Ty < 'tcx > > , dummy : PhantomData < TyCtxt < 'tcx > > , }
}