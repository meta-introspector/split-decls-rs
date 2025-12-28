macro_rules! deps {
    () => {
        ItemKind!();
        CtorKind!();
    };
}

macro_rules! new_item_kind {
    () => {
        deps!();
        pub (crate) fn new_item_kind (kind : DefKind) -> ItemKind { match kind { DefKind :: Mod | DefKind :: Struct | DefKind :: Union | DefKind :: Enum | DefKind :: Variant | DefKind :: Trait | DefKind :: TyAlias | DefKind :: ForeignTy | DefKind :: TraitAlias | DefKind :: AssocTy | DefKind :: TyParam | DefKind :: ConstParam | DefKind :: Macro (_) | DefKind :: ExternCrate | DefKind :: Use | DefKind :: ForeignMod | DefKind :: OpaqueTy | DefKind :: Field | DefKind :: LifetimeParam | DefKind :: Impl { .. } | DefKind :: GlobalAsm => { unreachable ! ("Not a valid item kind: {kind:?}") ; } DefKind :: Closure | DefKind :: AssocFn | DefKind :: Fn | DefKind :: SyntheticCoroutineBody => { ItemKind :: Fn } DefKind :: Const | DefKind :: InlineConst | DefKind :: AssocConst | DefKind :: AnonConst => { ItemKind :: Const } DefKind :: Static { .. } => ItemKind :: Static , DefKind :: Ctor (_ , rustc_hir :: def :: CtorKind :: Const) => ItemKind :: Ctor (CtorKind :: Const) , DefKind :: Ctor (_ , rustc_hir :: def :: CtorKind :: Fn) => ItemKind :: Ctor (CtorKind :: Fn) , } }
    };
}

new_item_kind!();