macro_rules! deps {
    () => {
        GenericParamDefKind!();
        Symbol!();
    };
}

macro_rules! GenericParamDef {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct GenericParamDef { pub name : super :: Symbol , pub def_id : GenericDef , pub index : u32 , pub pure_wrt_drop : bool , pub kind : GenericParamDefKind , }
    };
}

GenericParamDef!()