// Generated macro for build_clone_shim (function)
macro_rules! Depcrate_shimbuild_clone_shim {
() => {
// Module: crate::shim
// Provides: {"build_clone_shim"}
// Dependencies: {}
# [doc = " Builds a `Clone::clone` shim for `self_ty`. Here, `def_id` is `Clone::clone`."] fn build_clone_shim < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , self_ty : Ty < 'tcx >) -> Body < 'tcx > { debug ! ("build_clone_shim(def_id={:?})" , def_id) ; let mut builder = CloneShimBuilder :: new (tcx , def_id , self_ty) ; let dest = Place :: return_place () ; let src = tcx . mk_place_deref (Place :: from (Local :: new (1 + 0))) ; match self_ty . kind () { ty :: FnDef (..) | ty :: FnPtr (..) => builder . copy_shim () , ty :: Closure (_ , args) => builder . tuple_like_shim (dest , src , args . as_closure () . upvar_tys ()) , ty :: CoroutineClosure (_ , args) => { builder . tuple_like_shim (dest , src , args . as_coroutine_closure () . upvar_tys ()) } ty :: Tuple (..) => builder . tuple_like_shim (dest , src , self_ty . tuple_fields ()) , ty :: Coroutine (coroutine_def_id , args) => { assert_eq ! (tcx . coroutine_movability (* coroutine_def_id) , hir :: Movability :: Movable) ; builder . coroutine_shim (dest , src , * coroutine_def_id , args . as_coroutine ()) } _ => bug ! ("clone shim for `{:?}` which is not `Copy` and is not an aggregate" , self_ty) , } ; builder . into_mir () }
};
}
