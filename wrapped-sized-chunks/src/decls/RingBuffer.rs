macro_rules! deps {
    () => {
        Slice!();
        Chunk!();
        SliceMut!();
        RawIndex!();
    };
}

macro_rules! RingBuffer {
    () => {
        deps!();
        # [doc = " A fixed capacity ring buffer."] # [doc = ""] # [doc = " A ring buffer is an array where the first logical index is at some arbitrary"] # [doc = " location inside the array, and the indices wrap around to the start of the"] # [doc = " array once they overflow its bounds."] # [doc = ""] # [doc = " This gives us the ability to push to either the front or the end of the"] # [doc = " array in constant time, at the cost of losing the ability to get a single"] # [doc = " contiguous slice reference to the contents."] # [doc = ""] # [doc = " It differs from the [`Chunk`][Chunk] in that the latter will have mostly"] # [doc = " constant time pushes, but may occasionally need to shift its contents around"] # [doc = " to make room. They both have constant time pop, and they both have linear"] # [doc = " time insert and remove."] # [doc = ""] # [doc = " The `RingBuffer` offers its own [`Slice`][Slice] and [`SliceMut`][SliceMut]"] # [doc = " types to compensate for the loss of being able to take a slice, but they're"] # [doc = " somewhat less efficient, so the general rule should be that you shouldn't"] # [doc = " choose a `RingBuffer` if you rely heavily on slices - but if you don't,"] # [doc = " it's probably a marginally better choice overall than [`Chunk`][Chunk]."] # [doc = ""] # [doc = " # Feature Flag"] # [doc = ""] # [doc = " To use this data structure, you need to enable the `ringbuffer` feature."] # [doc = ""] # [doc = " [Chunk]: ../sized_chunk/struct.Chunk.html"] # [doc = " [Slice]: struct.Slice.html"] # [doc = " [SliceMut]: struct.SliceMut.html"] pub struct RingBuffer < A , const N : usize > { origin : RawIndex < N > , length : usize , data : MaybeUninit < [A ; N] > , }
    };
}

RingBuffer!()