macro_rules! deps {
    () => {
        SendSyncPhantomData!();
    };
}

macro_rules! SizeError {
    () => {
        deps!();
        # [doc = " The error emitted if the conversion source is of incorrect size."] pub struct SizeError < Src , Dst : ? Sized > { # [doc = " The source value involved in the conversion."] src : Src , # [doc = " The inner destination type involved in the conversion."] _dst : SendSyncPhantomData < Dst > , }
    };
}

SizeError!()