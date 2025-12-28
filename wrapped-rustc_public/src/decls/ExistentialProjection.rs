macro_rules! deps {
    () => {
        GenericArgs!();
        TermKind!();
    };
}

macro_rules! ExistentialProjection {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct ExistentialProjection { pub def_id : TraitDef , pub generic_args : GenericArgs , pub term : TermKind , }
    };
}

ExistentialProjection!();