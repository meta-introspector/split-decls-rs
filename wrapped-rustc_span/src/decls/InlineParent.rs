macro_rules! InlineParent {
    () => {
        # [derive (Clone , Copy)] struct InlineParent { lo : u32 , len_with_tag : u16 , parent : u16 , }
    };
}

InlineParent!();