macro_rules! deps {
    () => {
        ConvertError!();
        Validity!();
        Unaligned!();
        Immutable!();
        KnownLayout!();
        Alignment!();
        AlignmentError!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < Src , Dst : ? Sized + Unaligned , S , V > From < ConvertError < AlignmentError < Src , Dst > , S , V > > for ConvertError < Infallible , S , V > { # [doc = " Infallibly discards the alignment error from this `ConvertError` since"] # [doc = " `Dst` is unaligned."] # [doc = ""] # [doc = " Since [`Dst: Unaligned`], it is impossible to encounter an alignment"] # [doc = " error. This method permits discarding that alignment error infallibly"] # [doc = " and replacing it with [`Infallible`]."] # [doc = ""] # [doc = " [`Dst: Unaligned`]: crate::Unaligned"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::convert::Infallible;"] # [doc = " use zerocopy::*;"] # [doc = " # use zerocopy_derive::*;"] # [doc = ""] # [doc = " #[derive(TryFromBytes, KnownLayout, Unaligned, Immutable)]"] # [doc = " #[repr(C, packed)]"] # [doc = " struct Bools {"] # [doc = "     one: bool,"] # [doc = "     two: bool,"] # [doc = "     many: [bool],"] # [doc = " }"] # [doc = ""] # [doc = " impl Bools {"] # [doc = "     fn parse(bytes: &[u8]) -> Result<&Bools, AlignedTryCastError<&[u8], Bools>> {"] # [doc = "         // Since `Bools: Unaligned`, we can infallibly discard"] # [doc = "         // the alignment error."] # [doc = "         Bools::try_ref_from_bytes(bytes).map_err(Into::into)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [inline] fn from (err : ConvertError < AlignmentError < Src , Dst > , S , V >) -> ConvertError < Infallible , S , V > { match err { ConvertError :: Alignment (e) => { # [allow (unreachable_code)] return ConvertError :: Alignment (Infallible :: from (e)) ; } ConvertError :: Size (e) => ConvertError :: Size (e) , ConvertError :: Validity (e) => ConvertError :: Validity (e) , } } }
    };
}

impl_181!();