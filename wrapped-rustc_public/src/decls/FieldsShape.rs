macro_rules! deps {
    () => {
        Scalar!();
        Size!();
        Primitive!();
    };
}

macro_rules! FieldsShape {
    () => {
        deps!();
        # [doc = " Describes how the fields of a type are shaped in memory."] # [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub enum FieldsShape { # [doc = " Scalar primitives and `!`, which never have fields."] Primitive , # [doc = " All fields start at no offset. The `usize` is the field count."] Union (NonZero < usize >) , # [doc = " Array/vector-like placement, with all fields of identical types."] Array { stride : Size , count : u64 } , # [doc = " Struct-like placement, with precomputed offsets."] # [doc = ""] # [doc = " Fields are guaranteed to not overlap, but note that gaps"] # [doc = " before, between and after all the fields are NOT always"] # [doc = " padding, and as such their contents may not be discarded."] # [doc = " For example, enum variants leave a gap at the start,"] # [doc = " where the discriminant field in the enum layout goes."] Arbitrary { # [doc = " Offsets for the first byte of each field,"] # [doc = " ordered to match the source definition order."] # [doc = " I.e.: It follows the same order as [super::ty::VariantDef::fields()]."] # [doc = " This vector does not go in increasing order."] offsets : Vec < Size > , } , }
    };
}

FieldsShape!();