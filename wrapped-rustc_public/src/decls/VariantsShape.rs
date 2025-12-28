macro_rules! deps {
    () => {
        VariantIdx!();
        LayoutShape!();
        TagEncoding!();
        Scalar!();
    };
}

macro_rules! VariantsShape {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub enum VariantsShape { # [doc = " A type with no valid variants. Must be uninhabited."] Empty , # [doc = " Single enum variants, structs/tuples, unions, and all non-ADTs."] Single { index : VariantIdx } , # [doc = " Enum-likes with more than one inhabited variant: each variant comes with"] # [doc = " a *discriminant* (usually the same as the variant index but the user can"] # [doc = " assign explicit discriminant values). That discriminant is encoded"] # [doc = " as a *tag* on the machine. The layout of each variant is"] # [doc = " a struct, and they all have space reserved for the tag."] # [doc = " For enums, the tag is the sole field of the layout."] Multiple { tag : Scalar , tag_encoding : TagEncoding , tag_field : usize , variants : Vec < LayoutShape > , } , }
    };
}

VariantsShape!();