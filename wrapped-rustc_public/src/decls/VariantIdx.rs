macro_rules! VariantIdx {
    () => {
        # [doc = " The source-order index of a variant in a type."] # [doc = ""] # [doc = " For example, in the following types,"] # [doc = " ```ignore(illustrative)"] # [doc = " enum Demo1 {"] # [doc = "    Variant0 { a: bool, b: i32 },"] # [doc = "    Variant1 { c: u8, d: u64 },"] # [doc = " }"] # [doc = " struct Demo2 { e: u8, f: u16, g: u8 }"] # [doc = " ```"] # [doc = " `a` is in the variant with the `VariantIdx` of `0`,"] # [doc = " `c` is in the variant with the `VariantIdx` of `1`, and"] # [doc = " `g` is in the variant with the `VariantIdx` of `0`."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , Serialize)] pub struct VariantIdx (usize) ;
    };
}

VariantIdx!()