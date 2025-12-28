macro_rules! deps {
    () => {
        GenericArgs!();
    };
}

macro_rules! ExistentialTraitRef {
    () => {
        deps!();
        # [doc = " An existential reference to a trait where `Self` is not included."] # [doc = ""] # [doc = " The `generic_args` will include any other known argument."] # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct ExistentialTraitRef { pub def_id : TraitDef , pub generic_args : GenericArgs , }
    };
}

ExistentialTraitRef!()