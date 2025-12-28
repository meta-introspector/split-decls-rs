macro_rules! SmolStrBuilderRepr {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] enum SmolStrBuilderRepr { Inline { len : usize , buf : [u8 ; INLINE_CAP] } , Heap (String) , }
    };
}

SmolStrBuilderRepr!();