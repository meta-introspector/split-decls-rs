macro_rules! deps {
    () => {
        ConvertError!();
        TryFromBytes!();
        ValidityError!();
        SizeError!();
        Immutable!();
        TryCastError!();
        KnownLayout!();
        Unaligned!();
    };
}

macro_rules! AlignedTryCastError {
    () => {
        deps!();
        # [doc = " The error type of well-aligned, fallible casts."] # [doc = ""] # [doc = " This is like [`TryCastError`], but for casts that are always well-aligned."] # [doc = " It is identical to `TryCastError`, except that its alignment error is"] # [doc = " [`Infallible`]."] # [doc = ""] # [doc = " As of this writing, none of zerocopy's API produces this error directly."] # [doc = " However, it is useful since it permits users to infallibly discard alignment"] # [doc = " errors when they can prove statically that alignment errors are impossible."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::convert::Infallible;"] # [doc = " use zerocopy::*;"] # [doc = " # use zerocopy_derive::*;"] # [doc = ""] # [doc = " #[derive(TryFromBytes, KnownLayout, Unaligned, Immutable)]"] # [doc = " #[repr(C, packed)]"] # [doc = " struct Bools {"] # [doc = "     one: bool,"] # [doc = "     two: bool,"] # [doc = "     many: [bool],"] # [doc = " }"] # [doc = ""] # [doc = " impl Bools {"] # [doc = "     fn parse(bytes: &[u8]) -> Result<&Bools, AlignedTryCastError<&[u8], Bools>> {"] # [doc = "         // Since `Bools: Unaligned`, we can infallibly discard"] # [doc = "         // the alignment error."] # [doc = "         Bools::try_ref_from_bytes(bytes).map_err(Into::into)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [allow (type_alias_bounds)] pub type AlignedTryCastError < Src , Dst : ? Sized + TryFromBytes > = ConvertError < Infallible , SizeError < Src , Dst > , ValidityError < Src , Dst > > ;
    };
}

AlignedTryCastError!()