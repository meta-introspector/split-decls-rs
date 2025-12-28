macro_rules! deps {
    () => {
        VariantIdx!();
    };
}

macro_rules! VariantDef {
    () => {
        deps!();
        # [doc = " Definition of a variant, which can be either a struct / union field or an enum variant."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , Serialize)] pub struct VariantDef { # [doc = " The variant index."] # [doc = ""] # [doc = " ## Warning"] # [doc = " Do not access this field directly!"] pub idx : VariantIdx , # [doc = " The data type where this variant comes from."] # [doc = " For now, we use this to retrieve information about the variant itself so we don't need to"] # [doc = " cache more information."] # [doc = ""] # [doc = " ## Warning"] # [doc = " Do not access this field directly!"] pub adt_def : AdtDef , }
    };
}

VariantDef!();