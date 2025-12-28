macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! SliceVec {
    () => {
        deps!();
        # [doc = " A slice-backed vector-like data structure."] # [doc = ""] # [doc = " This is a very similar concept to `ArrayVec`, but instead"] # [doc = " of the backing memory being an owned array, the backing"] # [doc = " memory is a unique-borrowed slice. You can thus create"] # [doc = " one of these structures \"around\" some slice that you're"] # [doc = " working with to make it easier to manipulate."] # [doc = ""] # [doc = " * Has a fixed capacity (the initial slice size)."] # [doc = " * Has a variable length."] pub struct SliceVec < 's , T > { data : & 's mut [T] , len : usize , }
    };
}

SliceVec!();