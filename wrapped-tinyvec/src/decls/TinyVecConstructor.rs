macro_rules! deps {
    () => {
        TinyVec!();
        Array!();
        ArrayVec!();
    };
}

macro_rules! TinyVecConstructor {
    () => {
        deps!();
        # [doc (hidden)] pub enum TinyVecConstructor < A : Array > { Inline (fn (ArrayVec < A >) -> TinyVec < A >) , Heap (fn (Vec < A :: Item >) -> TinyVec < A >) , }
    };
}

TinyVecConstructor!();