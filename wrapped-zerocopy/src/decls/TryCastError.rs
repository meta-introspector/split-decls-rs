macro_rules! deps {
    () => {
        TryFromBytes!();
        ConvertError!();
        SizeError!();
        ValidityError!();
        AlignmentError!();
    };
}

macro_rules! TryCastError {
    () => {
        deps!();
        # [doc = " The error type of fallible reference conversions."] # [doc = ""] # [doc = " Fallible reference conversions, like [`TryFromBytes::try_ref_from_bytes`]"] # [doc = " may emit [alignment](AlignmentError), [size](SizeError), and"] # [doc = " [validity](ValidityError) errors."] # [allow (type_alias_bounds)] pub type TryCastError < Src , Dst : ? Sized + TryFromBytes > = ConvertError < AlignmentError < Src , Dst > , SizeError < Src , Dst > , ValidityError < Src , Dst > > ;
    };
}

TryCastError!()