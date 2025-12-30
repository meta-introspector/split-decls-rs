// Generated macro for Repr (enum)
macro_rules! DepcrateRepr {
() => {
// Module: crate
// Provides: {"Repr"}
// Dependencies: {}
# [derive (Clone , Debug)] enum Repr { Inline { len : InlineSize , buf : [u8 ; INLINE_CAP] } , Static (& 'static str) , Heap (Arc < str >) , }
};
}
