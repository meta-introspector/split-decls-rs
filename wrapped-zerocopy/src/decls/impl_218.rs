macro_rules! deps {
    () => {
        Validity!();
        TryCastError!();
        TryFromBytes!();
        Alignment!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl < Src , Dst : ? Sized + TryFromBytes > TryCastError < Src , Dst > { # [doc = " Produces the source underlying the failed conversion."] # [inline] pub fn into_src (self) -> Src { match self { Self :: Alignment (e) => e . src , Self :: Size (e) => e . src , Self :: Validity (e) => e . src , } } # [doc = " Maps the source value associated with the conversion error."] # [doc = ""] # [doc = " This can help mitigate [issues with `Send`, `Sync` and `'static`"] # [doc = " bounds][self#send-sync-and-static]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::num::NonZeroU32;"] # [doc = " use zerocopy::*;"] # [doc = ""] # [doc = " let source: [u8; 3] = [0, 0, 0];"] # [doc = ""] # [doc = " // Try to read a `NonZeroU32` from `source`."] # [doc = " let maybe_u32: Result<&NonZeroU32, TryCastError<&[u8], NonZeroU32>>"] # [doc = "     = NonZeroU32::try_ref_from_bytes(&source[..]);"] # [doc = ""] # [doc = " // Map the error's source to its size and address."] # [doc = " let maybe_u32: Result<&NonZeroU32, TryCastError<(usize, usize), NonZeroU32>> ="] # [doc = "     maybe_u32.map_err(|err| {"] # [doc = "         err.map_src(|src| (src.len(), src.as_ptr() as usize))"] # [doc = "     });"] # [doc = " ```"] # [inline] pub fn map_src < NewSrc > (self , f : impl FnOnce (Src) -> NewSrc) -> TryCastError < NewSrc , Dst > { match self { Self :: Alignment (e) => TryCastError :: Alignment (e . map_src (f)) , Self :: Size (e) => TryCastError :: Size (e . map_src (f)) , Self :: Validity (e) => TryCastError :: Validity (e . map_src (f)) , } } }
    };
}

impl_218!()