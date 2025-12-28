macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! TinyVecSplice {
    () => {
        deps!();
        # [doc = " Splicing iterator for `TinyVec`"] # [doc = " See [`TinyVec::splice`](TinyVec::<A>::splice)"] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub struct TinyVecSplice < 'p , A : Array , I : Iterator < Item = A :: Item > > { parent : & 'p mut TinyVec < A > , removal_start : usize , removal_end : usize , replacement : I , }
    };
}

TinyVecSplice!();