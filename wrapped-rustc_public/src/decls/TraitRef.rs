macro_rules! deps {
    () => {
        GenericArgs!();
    };
}

macro_rules! TraitRef {
    () => {
        deps!();
        # [doc = " A complete reference to a trait, i.e., one where `Self` is known."] # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct TraitRef { pub def_id : TraitDef , # [doc = " The generic arguments for this definition."] # [doc = " The first element must always be type, and it represents `Self`."] args : GenericArgs , }
    };
}

TraitRef!();