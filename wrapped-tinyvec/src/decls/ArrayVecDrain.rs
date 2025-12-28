macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! ArrayVecDrain {
    () => {
        deps!();
        # [doc = " Draining iterator for [`ArrayVec`]"] # [doc = ""] # [doc = " See [`ArrayVec::drain`](ArrayVec::drain)"] pub struct ArrayVecDrain < 'a , T : 'a + Default > { iter : slice :: IterMut < 'a , T > , }
    };
}

ArrayVecDrain!()