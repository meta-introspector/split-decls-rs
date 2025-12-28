macro_rules! deps {
    () => {
        GenericArgKind!();
    };
}

macro_rules! GenericArgs {
    () => {
        deps!();
        # [doc = " A list of generic arguments."] # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct GenericArgs (pub Vec < GenericArgKind >) ;
    };
}

GenericArgs!();