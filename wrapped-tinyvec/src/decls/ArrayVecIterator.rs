macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
    };
}

macro_rules! ArrayVecIterator {
    () => {
        deps!();
        # [doc = " Iterator for consuming an `ArrayVec` and returning owned elements."] pub struct ArrayVecIterator < A : Array > { base : u16 , tail : u16 , data : A , }
    };
}

ArrayVecIterator!()