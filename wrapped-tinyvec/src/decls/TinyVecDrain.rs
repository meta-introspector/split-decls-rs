macro_rules! deps {
    () => {
        ArrayVecDrain!();
        Array!();
    };
}

macro_rules! TinyVecDrain {
    () => {
        deps!();
        # [doc = " Draining iterator for `TinyVecDrain`"] # [doc = ""] # [doc = " See [`TinyVecDrain::drain`](TinyVecDrain::<A>::drain)"] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub enum TinyVecDrain < 'p , A : Array > { # [allow (missing_docs)] Inline (ArrayVecDrain < 'p , A :: Item >) , # [allow (missing_docs)] Heap (vec :: Drain < 'p , A :: Item >) , }
    };
}

TinyVecDrain!()