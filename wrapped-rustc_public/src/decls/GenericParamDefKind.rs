macro_rules! GenericParamDefKind {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum GenericParamDefKind { Lifetime , Type { has_default : bool , synthetic : bool } , Const { has_default : bool } , }
    };
}

GenericParamDefKind!()