macro_rules! deps {
    () => {
        IndexedVal!();
        Prov!();
        Allocation!();
        Error!();
    };
}

macro_rules! Bridge {
    () => {
        deps!();
        # [doc = " A trait defining types that are used to emulate rustc_public components, which is really"] # [doc = " useful when programming in rustc_public-agnostic settings."] pub trait Bridge : Sized { type DefId : Copy + Debug + PartialEq + IndexedVal ; type AllocId : Copy + Debug + PartialEq + IndexedVal ; type Span : Copy + Debug + PartialEq + IndexedVal ; type Ty : Copy + Debug + PartialEq + IndexedVal ; type InstanceDef : Copy + Debug + PartialEq + IndexedVal ; type TyConstId : Copy + Debug + PartialEq + IndexedVal ; type MirConstId : Copy + Debug + PartialEq + IndexedVal ; type Layout : Copy + Debug + PartialEq + IndexedVal ; type Error : Error ; type CrateItem : CrateItem < Self > ; type AdtDef : AdtDef < Self > ; type ForeignModuleDef : ForeignModuleDef < Self > ; type ForeignDef : ForeignDef < Self > ; type FnDef : FnDef < Self > ; type ClosureDef : ClosureDef < Self > ; type CoroutineDef : CoroutineDef < Self > ; type CoroutineClosureDef : CoroutineClosureDef < Self > ; type AliasDef : AliasDef < Self > ; type ParamDef : ParamDef < Self > ; type BrNamedDef : BrNamedDef < Self > ; type TraitDef : TraitDef < Self > ; type GenericDef : GenericDef < Self > ; type ConstDef : ConstDef < Self > ; type ImplDef : ImplDef < Self > ; type RegionDef : RegionDef < Self > ; type CoroutineWitnessDef : CoroutineWitnessDef < Self > ; type AssocDef : AssocDef < Self > ; type OpaqueDef : OpaqueDef < Self > ; type Prov : Prov < Self > ; type StaticDef : StaticDef < Self > ; type Allocation : Allocation < Self > ; }
    };
}

Bridge!();