macro_rules! deps {
    () => {
        DeflateStream!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < 'a > DeflateStream < 'a > { const _S : () = assert ! (core :: mem :: size_of ::< z_stream > () == core :: mem :: size_of ::< Self > ()) ; const _A : () = assert ! (core :: mem :: align_of ::< z_stream > () == core :: mem :: align_of ::< Self > ()) ; # [doc = " # Safety"] # [doc = ""] # [doc = " Behavior is undefined if any of the following conditions are violated:"] # [doc = ""] # [doc = " - `strm` satisfies the conditions of [`pointer::as_mut`]"] # [doc = " - if not `NULL`, `strm` as initialized using [`init`] or similar"] # [doc = ""] # [doc = " [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut"] # [inline (always)] pub unsafe fn from_stream_mut (strm : * mut z_stream) -> Option < & 'a mut Self > { { let stream = unsafe { strm . as_ref () } ? ; if stream . zalloc . is_none () || stream . zfree . is_none () { return None ; } if stream . state . is_null () { return None ; } } unsafe { strm . cast :: < DeflateStream > () . as_mut () } } # [doc = " # Safety"] # [doc = ""] # [doc = " Behavior is undefined if any of the following conditions are violated:"] # [doc = ""] # [doc = " - `strm` satisfies the conditions of [`pointer::as_ref`]"] # [doc = " - if not `NULL`, `strm` as initialized using [`init`] or similar"] # [doc = ""] # [doc = " [`pointer::as_ref`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_ref"] # [inline (always)] pub unsafe fn from_stream_ref (strm : * const z_stream) -> Option < & 'a Self > { { let stream = unsafe { strm . as_ref () } ? ; if stream . zalloc . is_none () || stream . zfree . is_none () { return None ; } if stream . state . is_null () { return None ; } } unsafe { strm . cast :: < DeflateStream > () . as_ref () } } fn as_z_stream_mut (& mut self) -> & mut z_stream { unsafe { & mut * (self as * mut DeflateStream as * mut z_stream) } } pub fn pending (& self) -> (usize , u8) { (self . state . bit_writer . pending . pending , self . state . bit_writer . bits_used ,) } }
    };
}

impl_121!();