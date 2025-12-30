// Generated macro for Wrap (struct)
macro_rules! DepcrateWrap {
() => {
// Module: crate
// Provides: {"Wrap"}
// Dependencies: {}
# [doc = " Wrapper that attaches context to a `Visitor`, `SeqAccess`, `EnumAccess` or"] # [doc = " `VariantAccess`."] struct Wrap < 'a , 'b , X , F : 'b > { delegate : X , callback : & 'b mut F , path : & 'a Path < 'a > , }
};
}
