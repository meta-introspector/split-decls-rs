macro_rules! EmbargoVisitor {
    () => {
        # [doc = " The embargo visitor, used to determine the exports of the AST."] struct EmbargoVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , # [doc = " Effective visibilities for reachable nodes."] effective_visibilities : EffectiveVisibilities , # [doc = " A set of pairs corresponding to modules, where the first module is"] # [doc = " reachable via a macro that's defined in the second module. This cannot"] # [doc = " be represented as reachable because it can't handle the following case:"] # [doc = ""] # [doc = " pub mod n {                         // Should be `Public`"] # [doc = "     pub(crate) mod p {              // Should *not* be accessible"] # [doc = "         pub fn f() -> i32 { 12 }    // Must be `Reachable`"] # [doc = "     }"] # [doc = " }"] # [doc = " pub macro m() {"] # [doc = "     n::p::f()"] # [doc = " }"] macro_reachable : FxHashSet < (LocalModDefId , LocalModDefId) > , # [doc = " Has something changed in the level map?"] changed : bool , }
    };
}

EmbargoVisitor!();