macro_rules! deps {
    () => {
        SendSyncPhantomData!();
        TryFromBytes!();
    };
}

macro_rules! ValidityError {
    () => {
        deps!();
        # [doc = " The error emitted if the conversion source contains invalid data."] pub struct ValidityError < Src , Dst : ? Sized + TryFromBytes > { # [doc = " The source value involved in the conversion."] pub (crate) src : Src , # [doc = " The inner destination type involved in the conversion."] _dst : SendSyncPhantomData < Dst > , }
    };
}

ValidityError!()