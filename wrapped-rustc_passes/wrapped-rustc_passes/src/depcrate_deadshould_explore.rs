// Generated macro for should_explore (function)
macro_rules! Depcrate_deadshould_explore {
() => {
// Module: crate::dead
// Provides: {"should_explore"}
// Dependencies: {}
# [doc = " Any local definition that may call something in its body block should be explored. For example,"] # [doc = " if it's a live function, then we should explore its block to check for codes that may need to"] # [doc = " be marked as live."] fn should_explore (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { match tcx . def_kind (def_id) { DefKind :: Mod | DefKind :: Struct | DefKind :: Union | DefKind :: Enum | DefKind :: Variant | DefKind :: Trait | DefKind :: TyAlias | DefKind :: ForeignTy | DefKind :: TraitAlias | DefKind :: AssocTy | DefKind :: Fn | DefKind :: Const | DefKind :: Static { .. } | DefKind :: AssocFn | DefKind :: AssocConst | DefKind :: Macro (_) | DefKind :: GlobalAsm | DefKind :: Impl { .. } | DefKind :: OpaqueTy | DefKind :: AnonConst | DefKind :: InlineConst | DefKind :: ExternCrate | DefKind :: Use | DefKind :: Ctor (..) | DefKind :: ForeignMod => true , DefKind :: TyParam | DefKind :: ConstParam | DefKind :: Field | DefKind :: LifetimeParam | DefKind :: Closure | DefKind :: SyntheticCoroutineBody => false , } }
};
}
