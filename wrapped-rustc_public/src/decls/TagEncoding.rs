macro_rules! deps {
    () => {
        VariantIdx!();
    };
}

macro_rules! TagEncoding {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub enum TagEncoding { # [doc = " The tag directly stores the discriminant, but possibly with a smaller layout"] # [doc = " (so converting the tag to the discriminant can require sign extension)."] Direct , # [doc = " Niche (values invalid for a type) encoding the discriminant:"] # [doc = " Discriminant and variant index coincide."] # [doc = " The variant `untagged_variant` contains a niche at an arbitrary"] # [doc = " offset (field `tag_field` of the enum), which for a variant with"] # [doc = " discriminant `d` is set to"] # [doc = " `(d - niche_variants.start).wrapping_add(niche_start)`."] # [doc = ""] # [doc = " For example, `Option<(usize, &T)>`  is represented such that"] # [doc = " `None` has a null pointer for the second tuple field, and"] # [doc = " `Some` is the identity function (with a non-null reference)."] Niche { untagged_variant : VariantIdx , niche_variants : RangeInclusive < VariantIdx > , niche_start : u128 , } , }
    };
}

TagEncoding!();