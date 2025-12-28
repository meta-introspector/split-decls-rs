macro_rules! deps {
    () => {
        SparseChunk!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < A , const N : usize > PartialEq < BTreeMap < usize , A > > for SparseChunk < A , N > where A : PartialEq , BitsImpl < N > : Bits , { fn eq (& self , other : & BTreeMap < usize , A >) -> bool { if self . len () != other . len () { return false ; } for index in self . indices () { if self . get (index) != other . get (& index) { return false ; } } true } }
    };
}

impl_111!();