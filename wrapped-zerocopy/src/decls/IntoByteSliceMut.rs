macro_rules! deps {
    () => {
        IntoByteSlice!();
        ByteSliceMut!();
        ByteSlice!();
    };
}

macro_rules! IntoByteSliceMut {
    () => {
        deps!();
        # [allow (clippy :: missing_safety_doc)] # [doc = " A [`ByteSliceMut`] that conveys no ownership, and so can be converted into a"] # [doc = " mutable byte slice."] # [doc = ""] # [doc = " Some `ByteSliceMut` types (notably, the standard library's [`RefMut`] type)"] # [doc = " convey ownership, and so they cannot soundly be moved by-value into a byte"] # [doc = " slice type (`&mut [u8]`). Some methods in this crate's API (such as"] # [doc = " [`Ref::into_mut`]) are only compatible with `ByteSliceMut` types without"] # [doc = " these ownership semantics."] # [doc = ""] # [doc = " [`RefMut`]: core::cell::RefMut"] pub unsafe trait IntoByteSliceMut < 'a > : IntoByteSlice < 'a > + ByteSliceMut { # [doc = " Coverts `self` into a `&mut [u8]`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The returned reference has the same address and length as `self.deref()`"] # [doc = " and `self.deref_mut()`."] # [doc = ""] # [doc = " Note that, combined with the safety invariant on [`ByteSlice`], this"] # [doc = " safety invariant implies that the returned reference is \"stable\" in the"] # [doc = " sense described in the `ByteSlice` docs."] fn into_byte_slice_mut (self) -> & 'a mut [u8] ; }
    };
}

IntoByteSliceMut!()