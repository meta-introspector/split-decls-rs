macro_rules! deps {
    () => {
        NetworkEndian!();
        Order!();
        LE!();
        BigEndian!();
        BE!();
        LittleEndian!();
        NativeEndian!();
    };
}

macro_rules! ByteOrder {
    () => {
        deps!();
        # [doc = " A type-level representation of byte order."] # [doc = ""] # [doc = " This type is implemented by [`BigEndian`] and [`LittleEndian`], which"] # [doc = " represent big-endian and little-endian byte order respectively. This module"] # [doc = " also provides a number of useful aliases for those types: [`NativeEndian`],"] # [doc = " [`NetworkEndian`], [`BE`], and [`LE`]."] # [doc = ""] # [doc = " `ByteOrder` types can be used to specify the byte order of the types in this"] # [doc = " module - for example, [`U32<BigEndian>`] is a 32-bit integer stored in"] # [doc = " big-endian byte order."] # [doc = ""] # [doc = " [`U32<BigEndian>`]: U32"] pub trait ByteOrder : Copy + Clone + Debug + Display + Eq + PartialEq + Ord + PartialOrd + Hash + private :: Sealed { # [doc (hidden)] const ORDER : Order ; }
    };
}

ByteOrder!()