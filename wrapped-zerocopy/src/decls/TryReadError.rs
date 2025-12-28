macro_rules! deps {
    () => {
        TryFromBytes!();
        ConvertError!();
        SizeError!();
        ValidityError!();
    };
}

macro_rules! TryReadError {
    () => {
        deps!();
        # [doc = " The error type of fallible read-conversions."] # [doc = ""] # [doc = " Fallible read-conversions, like [`TryFromBytes::try_read_from_bytes`] may emit"] # [doc = " [size](SizeError) and [validity](ValidityError) errors, but not alignment errors."] # [allow (type_alias_bounds)] pub type TryReadError < Src , Dst : ? Sized + TryFromBytes > = ConvertError < Infallible , SizeError < Src , Dst > , ValidityError < Src , Dst > > ;
    };
}

TryReadError!();