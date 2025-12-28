macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! SliceVecDrain {
    () => {
        deps!();
        # [doc = " Draining iterator for [`SliceVec`]"] # [doc = ""] # [doc = " See [`SliceVec::drain`](SliceVec::drain)"] pub struct SliceVecDrain < 'p , 's , T : Default > { parent : & 'p mut SliceVec < 's , T > , target_start : usize , target_index : usize , target_end : usize , }
    };
}

SliceVecDrain!()