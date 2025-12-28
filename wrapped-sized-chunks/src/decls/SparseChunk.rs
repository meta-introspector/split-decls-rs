macro_rules! SparseChunk {
    () => {
        # [doc = " A fixed capacity sparse array."] # [doc = ""] # [doc = " An inline sparse array of up to `N` items of type `A`. You can think of it as an array"] # [doc = " of `Option<A>`, where the discriminant (whether the value is `Some<A>` or"] # [doc = " `None`) is kept in a bitmap instead of adjacent to the value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use sized_chunks::SparseChunk;"] # [doc = " // Construct a chunk with a 20 item capacity"] # [doc = " let mut chunk = SparseChunk::<i32, 20>::new();"] # [doc = " // Set the 18th index to the value 5."] # [doc = " chunk.insert(18, 5);"] # [doc = " // Set the 5th index to the value 23."] # [doc = " chunk.insert(5, 23);"] # [doc = ""] # [doc = " assert_eq!(chunk.len(), 2);"] # [doc = " assert_eq!(chunk.get(5), Some(&23));"] # [doc = " assert_eq!(chunk.get(6), None);"] # [doc = " assert_eq!(chunk.get(18), Some(&5));"] # [doc = " ```"] pub struct SparseChunk < A , const N : usize > where BitsImpl < N > : Bits , { map : Bitmap < N > , data : MaybeUninit < [A ; N] > , }
    };
}

SparseChunk!();