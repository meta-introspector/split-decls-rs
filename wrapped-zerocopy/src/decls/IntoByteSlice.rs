macro_rules! deps {
    () => {
        ByteSlice!();
    };
}

macro_rules! IntoByteSlice {
    () => {
        deps!();
        # [allow (clippy :: missing_safety_doc)] # [doc = " A [`ByteSlice`] that conveys no ownership, and so can be converted into a"] # [doc = " byte slice."] # [doc = ""] # [doc = " Some `ByteSlice` types (notably, the standard library's [`Ref`] type) convey"] # [doc = " ownership, and so they cannot soundly be moved by-value into a byte slice"] # [doc = " type (`&[u8]`). Some methods in this crate's API (such as [`Ref::into_ref`])"] # [doc = " are only compatible with `ByteSlice` types without these ownership"] # [doc = " semantics."] # [doc = ""] # [doc = " [`Ref`]: core::cell::Ref"] pub unsafe trait IntoByteSlice < 'a > : ByteSlice { # [doc = " Coverts `self` into a `&[u8]`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The returned reference has the same address and length as `self.deref()`"] # [doc = " and `self.deref_mut()`."] # [doc = ""] # [doc = " Note that, combined with the safety invariant on [`ByteSlice`], this"] # [doc = " safety invariant implies that the returned reference is \"stable\" in the"] # [doc = " sense described in the `ByteSlice` docs."] fn into_byte_slice (self) -> & 'a [u8] ; }
    };
}

IntoByteSlice!();