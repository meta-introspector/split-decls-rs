macro_rules! deps {
    () => {
        SendSyncPhantomData!();
    };
}

macro_rules! AlignmentError {
    () => {
        deps!();
        # [doc = " The error emitted if the conversion source is improperly aligned."] pub struct AlignmentError < Src , Dst : ? Sized > { # [doc = " The source value involved in the conversion."] src : Src , # [doc = " The inner destination type involved in the conversion."] # [doc = ""] # [doc = " INVARIANT: An `AlignmentError` may only be constructed if `Dst`'s"] # [doc = " alignment requirement is greater than one."] _dst : SendSyncPhantomData < Dst > , }
    };
}

AlignmentError!();