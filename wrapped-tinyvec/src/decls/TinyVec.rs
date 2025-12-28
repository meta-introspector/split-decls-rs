macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! TinyVec {
    () => {
        deps!();
        # [doc = " A vector that starts inline, but can automatically move to the heap."] # [doc = ""] # [doc = " * Requires the `alloc` feature"] # [doc = ""] # [doc = " A `TinyVec` is either an Inline([`ArrayVec`](crate::ArrayVec::<A>)) or"] # [doc = " Heap([`Vec`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html)). The"] # [doc = " interface for the type as a whole is a bunch of methods that just match on"] # [doc = " the enum variant and then call the same method on the inner vec."] # [doc = ""] # [doc = " ## Construction"] # [doc = ""] # [doc = " Because it's an enum, you can construct a `TinyVec` simply by making an"] # [doc = " `ArrayVec` or `Vec` and then putting it into the enum."] # [doc = ""] # [doc = " There is also a macro"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use tinyvec::*;"] # [doc = " let empty_tv = tiny_vec!([u8; 16]);"] # [doc = " let some_ints = tiny_vec!([i32; 4] => 1, 2, 3);"] # [doc = " ```"] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub enum TinyVec < A : Array > { # [allow (missing_docs)] Inline (ArrayVec < A >) , # [allow (missing_docs)] Heap (Vec < A :: Item >) , }
    };
}

TinyVec!()