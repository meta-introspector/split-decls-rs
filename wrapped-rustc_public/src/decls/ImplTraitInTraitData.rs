macro_rules! ImplTraitInTraitData {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Serialize)] pub enum ImplTraitInTraitData { Trait { fn_def_id : FnDef , opaque_def_id : OpaqueDef } , Impl { fn_def_id : FnDef } , }
    };
}

ImplTraitInTraitData!()