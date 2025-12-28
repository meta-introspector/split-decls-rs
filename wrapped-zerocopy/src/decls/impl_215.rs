macro_rules! deps {
    () => {
        ConvertError!();
        TryCastError!();
        TryFromBytes!();
        Alignment!();
        CastError!();
        Validity!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl < Src , Dst : ? Sized > CastError < Src , Dst > { # [doc = " Produces the source underlying the failed conversion."] # [inline] pub fn into_src (self) -> Src { match self { Self :: Alignment (e) => e . src , Self :: Size (e) => e . src , Self :: Validity (i) => match i { } , } } # [doc = " Sets the source value associated with the conversion error."] pub (crate) fn with_src < NewSrc > (self , new_src : NewSrc) -> CastError < NewSrc , Dst > { match self { Self :: Alignment (e) => CastError :: Alignment (e . with_src (new_src)) , Self :: Size (e) => CastError :: Size (e . with_src (new_src)) , Self :: Validity (i) => match i { } , } } # [doc = " Maps the source value associated with the conversion error."] # [doc = ""] # [doc = " This can help mitigate [issues with `Send`, `Sync` and `'static`"] # [doc = " bounds][self#send-sync-and-static]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use zerocopy::*;"] # [doc = ""] # [doc = " let source: [u8; 3] = [0, 1, 2];"] # [doc = ""] # [doc = " // Try to read a `u32` from `source`. This will fail because there are insufficient"] # [doc = " // bytes in `source`."] # [doc = " let maybe_u32: Result<&u32, CastError<&[u8], u32>> = u32::ref_from_bytes(&source[..]);"] # [doc = ""] # [doc = " // Map the error's source to its size and address."] # [doc = " let maybe_u32: Result<&u32, CastError<(usize, usize), u32>> = maybe_u32.map_err(|err| {"] # [doc = "     err.map_src(|src| (src.len(), src.as_ptr() as usize))"] # [doc = " });"] # [doc = " ```"] # [inline] pub fn map_src < NewSrc > (self , f : impl FnOnce (Src) -> NewSrc) -> CastError < NewSrc , Dst > { match self { Self :: Alignment (e) => CastError :: Alignment (e . map_src (f)) , Self :: Size (e) => CastError :: Size (e . map_src (f)) , Self :: Validity (i) => match i { } , } } # [doc = " Converts the error into a general [`ConvertError`]."] pub (crate) fn into (self) -> TryCastError < Src , Dst > where Dst : TryFromBytes , { match self { Self :: Alignment (e) => TryCastError :: Alignment (e) , Self :: Size (e) => TryCastError :: Size (e) , Self :: Validity (i) => match i { } , } } }
    };
}

impl_215!();