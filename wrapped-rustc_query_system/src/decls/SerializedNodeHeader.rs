macro_rules! deps {
    () => {
        DepNode!();
        NodeInfo!();
    };
}

macro_rules! SerializedNodeHeader {
    () => {
        deps!();
        # [doc = " A packed representation of all the fixed-size fields in a `NodeInfo`."] # [doc = ""] # [doc = " This stores in one byte array:"] # [doc = " * The `Fingerprint` in the `NodeInfo`"] # [doc = " * The `Fingerprint` in `DepNode` that is in this `NodeInfo`"] # [doc = " * The `DepKind`'s discriminant (a u16, but not all bits are used...)"] # [doc = " * The byte width of the encoded edges for this node"] # [doc = " * In whatever bits remain, the length of the edge list for this node, if it fits"] struct SerializedNodeHeader < D > { bytes : [u8 ; 38] , _marker : PhantomData < D > , }
    };
}

SerializedNodeHeader!();