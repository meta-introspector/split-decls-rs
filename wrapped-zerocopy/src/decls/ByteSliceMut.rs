macro_rules! deps {
    () => {
        ByteSlice!();
    };
}

macro_rules! ByteSliceMut {
    () => {
        deps!();
        # [doc = " A mutable reference to a byte slice."] # [doc = ""] # [doc = " `ByteSliceMut` abstracts over various ways of storing a mutable reference to"] # [doc = " a byte slice, and is implemented for various special reference types such as"] # [doc = " `RefMut<[u8]>`."] # [doc = ""] # [doc = " `ByteSliceMut` is a shorthand for [`ByteSlice`] and [`DerefMut`]."] pub trait ByteSliceMut : ByteSlice + DerefMut { }
    };
}

ByteSliceMut!()