macro_rules! deps {
    () => {
        Variant!();
    };
}

macro_rules! ProjectedUserTypesOp {
    () => {
        deps!();
        # [doc = " One of a list of \"operations\" that can be used to lazily build projections"] # [doc = " of user-specified types."] # [derive (Clone , Debug)] pub (crate) enum ProjectedUserTypesOp { PushUserType { base : UserTypeAnnotationIndex } , Index , Subslice { from : u64 , to : u64 } , Deref , Leaf { field : FieldIdx } , Variant { name : Symbol , variant : VariantIdx , field : FieldIdx } , }
    };
}

ProjectedUserTypesOp!()