macro_rules! deps {
    () => {
        SizeError!();
        SendSyncPhantomData!();
        ConvertError!();
        SizeInfo!();
        KnownLayout!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl < Src , Dst : ? Sized > SizeError < Src , Dst > { pub (crate) fn new (src : Src) -> Self { Self { src , _dst : SendSyncPhantomData :: default () } } # [doc = " Produces the source underlying the failed conversion."] # [inline] pub fn into_src (self) -> Src { self . src } # [doc = " Sets the source value associated with the conversion error."] pub (crate) fn with_src < NewSrc > (self , new_src : NewSrc) -> SizeError < NewSrc , Dst > { SizeError { src : new_src , _dst : SendSyncPhantomData :: default () } } # [doc = " Maps the source value associated with the conversion error."] # [doc = ""] # [doc = " This can help mitigate [issues with `Send`, `Sync` and `'static`"] # [doc = " bounds][self#send-sync-and-static]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use zerocopy::*;"] # [doc = ""] # [doc = " let source: [u8; 3] = [0, 1, 2];"] # [doc = ""] # [doc = " // Try to read a `u32` from `source`. This will fail because there are insufficient"] # [doc = " // bytes in `source`."] # [doc = " let maybe_u32: Result<u32, SizeError<&[u8], u32>> = u32::read_from_bytes(&source[..]);"] # [doc = ""] # [doc = " // Map the error's source to its size."] # [doc = " let maybe_u32: Result<u32, SizeError<usize, u32>> = maybe_u32.map_err(|err| {"] # [doc = "     err.map_src(|src| src.len())"] # [doc = " });"] # [doc = " ```"] # [inline] pub fn map_src < NewSrc > (self , f : impl FnOnce (Src) -> NewSrc) -> SizeError < NewSrc , Dst > { SizeError { src : f (self . src) , _dst : SendSyncPhantomData :: default () } } # [doc = " Sets the destination type associated with the conversion error."] pub (crate) fn with_dst < NewDst : ? Sized > (self) -> SizeError < Src , NewDst > { SizeError { src : self . src , _dst : SendSyncPhantomData :: default () } } # [doc = " Converts the error into a general [`ConvertError`]."] pub (crate) fn into < A , V > (self) -> ConvertError < A , Self , V > { ConvertError :: Size (self) } # [doc = " Format extra details for a verbose, human-readable error message."] # [doc = ""] # [doc = " This formatting may include potentially sensitive information."] fn display_verbose_extras (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result where Src : Deref , Dst : KnownLayout , { f . write_str ("\nSource type: ") ? ; f . write_str (core :: any :: type_name :: < Src > ()) ? ; let src_size = core :: mem :: size_of_val (& * self . src) ; f . write_str ("\nSource size: ") ? ; src_size . fmt (f) ? ; f . write_str (" byte") ? ; if src_size != 1 { f . write_char ('s') ? ; } if let crate :: SizeInfo :: Sized { size } = Dst :: LAYOUT . size_info { f . write_str ("\nDestination size: ") ? ; size . fmt (f) ? ; f . write_str (" byte") ? ; if size != 1 { f . write_char ('s') ? ; } } f . write_str ("\nDestination type: ") ? ; f . write_str (core :: any :: type_name :: < Dst > ()) ? ; Ok (()) } }
    };
}

impl_197!()