macro_rules! POINTER_WIDTH_BITS {
    () => {
        # [doc = " The target pointer width, counted in bits."] const POINTER_WIDTH_BITS : usize = mem :: size_of :: < usize > () * 8 ;
    };
}

POINTER_WIDTH_BITS!()