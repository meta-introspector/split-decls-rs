macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! ArrayVecSplice {
    () => {
        deps!();
        # [doc = " Splicing iterator for `ArrayVec`"] # [doc = " See [`ArrayVec::splice`](ArrayVec::<A>::splice)"] pub struct ArrayVecSplice < 'p , A : Array , I : Iterator < Item = A :: Item > > { parent : & 'p mut ArrayVec < A > , removal_start : usize , removal_end : usize , replacement : I , }
    };
}

ArrayVecSplice!()