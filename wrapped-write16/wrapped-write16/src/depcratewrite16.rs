// Generated macro for Write16 (trait)
macro_rules! DepcrateWrite16 {
() => {
// Module: crate
// Provides: {"Write16"}
// Dependencies: {}
# [doc = " A UTF-16 sink analogous to `core::fmt::Write`."] pub trait Write16 { # [doc = " Write a slice containing UTF-16 to the sink."] # [doc = ""] # [doc = " The implementor of the trait should not validate UTF-16."] # [doc = " It's the responsibility of the caller to pass valid"] # [doc = " UTF-16."] fn write_slice (& mut self , s : & [u16]) -> core :: fmt :: Result ; # [doc = " Write a Unicode scalar value to the sink."] # [inline (always)] fn write_char (& mut self , c : char) -> core :: fmt :: Result { let mut buf = [0u16 ; 2] ; self . write_slice (c . encode_utf16 (& mut buf)) } # [doc = " A hint that the caller expects to write `upcoming` UTF-16"] # [doc = " code units. The implementation must not assume `upcoming`"] # [doc = " to be exact. The caller may write more or fewer code units"] # [doc = " using `write_slice()` and `write_char()`. However, the"] # [doc = " caller should try to give reasonable estimates if it uses"] # [doc = " this method."] # [doc = ""] # [doc = " For `Vec` and `SmallVec`, this maps to `reserve()`."] # [doc = " The default implementation does nothing."] # [inline (always)] fn size_hint (& mut self , upcoming : usize) -> core :: fmt :: Result { let _ = upcoming ; Ok (()) } }
};
}
