macro_rules! deps {
    () => {
        ByteSliceMut!();
        IntoByteSliceMut!();
        IntoByteSlice!();
    };
}

macro_rules! ByteSlice {
    () => {
        deps!();
        # [doc = " A mutable or immutable reference to a byte slice."] # [doc = ""] # [doc = " `ByteSlice` abstracts over the mutability of a byte slice reference, and is"] # [doc = " implemented for various special reference types such as"] # [doc = " [`Ref<[u8]>`](core::cell::Ref) and [`RefMut<[u8]>`](core::cell::RefMut)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Implementations of `ByteSlice` must promise that their implementations of"] # [doc = " [`Deref`] and [`DerefMut`] are \"stable\". In particular, given `B: ByteSlice`"] # [doc = " and `b: B`, two calls, each to either `b.deref()` or `b.deref_mut()`, must"] # [doc = " return a byte slice with the same address and length. This must hold even if"] # [doc = " the two calls are separated by an arbitrary sequence of calls to methods on"] # [doc = " `ByteSlice`, [`ByteSliceMut`], [`IntoByteSlice`], or [`IntoByteSliceMut`],"] # [doc = " or on their super-traits. This does *not* need to hold if the two calls are"] # [doc = " separated by any method calls, field accesses, or field modifications *other"] # [doc = " than* those from these traits."] # [doc = ""] # [doc = " Note that this also implies that, given `b: B`, the address and length"] # [doc = " cannot be modified via objects other than `b`, either on the same thread or"] # [doc = " on another thread."] pub unsafe trait ByteSlice : Deref < Target = [u8] > + Sized { }
    };
}

ByteSlice!()