// Generated macro for private (module)
macro_rules! Depcrate_bufferprivate {
() => {
// Module: crate::buffer
// Provides: {"private"}
// Dependencies: {}
mod private { pub trait Sealed < T > { # [doc = " The result of the process operation."] type Output ; # [doc = " Return a pointer and length for this buffer."] # [doc = ""] # [doc = " The length is the number of elements of type `T`, not a number of"] # [doc = " bytes."] # [doc = ""] # [doc = " It's tempting to have this return `&mut [MaybeUninit<T>]` instead,"] # [doc = " however that would require this function to be `unsafe`, because"] # [doc = " callers could use the `&mut [MaybeUninit<T>]` slice to set elements"] # [doc = " to `MaybeUninit::<T>::uninit()`, which would be a problem if `Self`"] # [doc = " is `&mut [T]` or similar."] fn parts_mut (& mut self) -> (* mut T , usize) ; # [doc = " Convert a finished buffer pointer into its result."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " At least `len` elements of the buffer must now be initialized."] # [must_use] unsafe fn assume_init (self , len : usize) -> Self :: Output ; } }
};
}
