macro_rules! deps {
    () => {
        BridgeTys!();
        DefId!();
        Error!();
        MirConstId!();
        Allocation!();
        Prov!();
        TyConstId!();
        Layout!();
        Span!();
        Ty!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl Bridge for BridgeTys { type DefId = crate :: DefId ; type AllocId = crate :: mir :: alloc :: AllocId ; type Span = crate :: ty :: Span ; type Ty = crate :: ty :: Ty ; type InstanceDef = crate :: mir :: mono :: InstanceDef ; type TyConstId = crate :: ty :: TyConstId ; type MirConstId = crate :: ty :: MirConstId ; type Layout = crate :: abi :: Layout ; type Error = crate :: Error ; type CrateItem = crate :: CrateItem ; type AdtDef = crate :: ty :: AdtDef ; type ForeignModuleDef = crate :: ty :: ForeignModuleDef ; type ForeignDef = crate :: ty :: ForeignDef ; type FnDef = crate :: ty :: FnDef ; type ClosureDef = crate :: ty :: ClosureDef ; type CoroutineDef = crate :: ty :: CoroutineDef ; type CoroutineClosureDef = crate :: ty :: CoroutineClosureDef ; type AliasDef = crate :: ty :: AliasDef ; type ParamDef = crate :: ty :: ParamDef ; type BrNamedDef = crate :: ty :: BrNamedDef ; type TraitDef = crate :: ty :: TraitDef ; type GenericDef = crate :: ty :: GenericDef ; type ConstDef = crate :: ty :: ConstDef ; type ImplDef = crate :: ty :: ImplDef ; type RegionDef = crate :: ty :: RegionDef ; type CoroutineWitnessDef = crate :: ty :: CoroutineWitnessDef ; type AssocDef = crate :: ty :: AssocDef ; type OpaqueDef = crate :: ty :: OpaqueDef ; type Prov = crate :: ty :: Prov ; type StaticDef = crate :: mir :: mono :: StaticDef ; type Allocation = crate :: ty :: Allocation ; }
    };
}

impl_263!();