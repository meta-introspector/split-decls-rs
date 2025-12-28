macro_rules! deps {
    () => {
        SplitByteSlice!();
        ByteSliceMut!();
    };
}

macro_rules! SplitByteSliceMut {
    () => {
        deps!();
        # [doc = " A shorthand for [`SplitByteSlice`] and [`ByteSliceMut`]."] pub trait SplitByteSliceMut : SplitByteSlice + ByteSliceMut { }
    };
}

SplitByteSliceMut!()