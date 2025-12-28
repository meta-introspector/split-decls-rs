macro_rules! deps {
    () => {
        InlineSize!();
    };
}

macro_rules! Repr {
    () => {
        deps!();
        # [derive (Clone , Debug)] enum Repr { Inline { len : InlineSize , buf : [u8 ; INLINE_CAP] } , Static (& 'static str) , Heap (Arc < str >) , }
    };
}

Repr!();