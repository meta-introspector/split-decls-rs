macro_rules! deps {
    () => {
        ByteSliceMut!();
        SplitByteSlice!();
    };
}

macro_rules! SplitByteSliceMut {
    () => {
        deps!();
        # [doc = " A shorthand for [`SplitByteSlice`] and [`ByteSliceMut`]."] pub trait SplitByteSliceMut : SplitByteSlice + ByteSliceMut { }
    };
}

SplitByteSliceMut!();