// Generated macro for ImplTraitInTraitFinder (struct)
macro_rules! Depcrate_tyImplTraitInTraitFinder {
() => {
// Module: crate::ty
// Provides: {"ImplTraitInTraitFinder"}
// Dependencies: {}
# [doc = " Walk through a function type, gathering all RPITITs and installing a"] # [doc = " `NormalizesTo(Projection(RPITIT) -> Opaque(RPITIT))` predicate into the"] # [doc = " predicates list. This allows us to observe that an RPITIT projects to"] # [doc = " its corresponding opaque within the body of a default-body trait method."] struct ImplTraitInTraitFinder < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , predicates : & 'a mut Vec < ty :: Clause < 'tcx > > , fn_def_id : DefId , bound_vars : & 'tcx ty :: List < ty :: BoundVariableKind > , seen : FxHashSet < DefId > , depth : ty :: DebruijnIndex , }
};
}
