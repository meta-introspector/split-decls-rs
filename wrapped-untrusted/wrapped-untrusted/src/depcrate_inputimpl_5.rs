// Generated macro for impl_5 (impl)
macro_rules! Depcrate_inputimpl_5 {
() => {
// Module: crate::input
// Provides: {"impl_5"}
// Dependencies: {}
impl < 'a > Input < 'a > { # [doc = " Construct a new `Input` for the given input `bytes`."] pub const fn from (bytes : & 'a [u8]) -> Self { Self { value : no_panic :: Slice :: new (bytes) , } } # [doc = " Returns `true` if the input is empty and false otherwise."] # [inline] pub fn is_empty (& self) -> bool { self . value . is_empty () } # [doc = " Returns the length of the `Input`."] # [inline] pub fn len (& self) -> usize { self . value . len () } # [doc = " Calls `read` with the given input as a `Reader`, ensuring that `read`"] # [doc = " consumed the entire input. If `read` does not consume the entire input,"] # [doc = " `incomplete_read` is returned."] pub fn read_all < F , R , E > (& self , incomplete_read : E , read : F) -> Result < R , E > where F : FnOnce (& mut Reader < 'a >) -> Result < R , E > , { let mut input = Reader :: new (* self) ; let result = read (& mut input) ? ; if input . at_end () { Ok (result) } else { Err (incomplete_read) } } # [doc = " Access the input as a slice so it can be processed by functions that"] # [doc = " are not written using the Input/Reader framework."] # [inline] pub fn as_slice_less_safe (& self) -> & 'a [u8] { self . value . as_slice_less_safe () } pub (super) fn into_value (self) -> no_panic :: Slice < 'a > { self . value } }
};
}
