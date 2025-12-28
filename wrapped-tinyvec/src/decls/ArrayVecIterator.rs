macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! ArrayVecIterator {
    () => {
        deps!();
        # [doc = " Iterator for consuming an `ArrayVec` and returning owned elements."] pub struct ArrayVecIterator < A : Array > { base : u16 , tail : u16 , data : A , }
    };
}

ArrayVecIterator!();