macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < A : Array > Into < Vec < A :: Item > > for TinyVec < A > { # [doc = " Converts a `TinyVec` into a `Vec`."] # [doc = ""] # [doc = " ## Examples"] # [doc = ""] # [doc = " ### Inline to Vec"] # [doc = ""] # [doc = " For `TinyVec::Inline(_)`,"] # [doc = "   `.into()` **does not** offer a performance advantage over `.to_vec()`."] # [doc = ""] # [doc = " ```"] # [doc = " use core::mem::size_of_val as mem_size_of;"] # [doc = " use tinyvec::TinyVec;"] # [doc = ""] # [doc = " let v = TinyVec::from([0u8; 128]);"] # [doc = " assert_eq!(mem_size_of(&v), 136);"] # [doc = ""] # [doc = " let vec: Vec<_> = v.into();"] # [doc = " assert_eq!(mem_size_of(&vec), 24);"] # [doc = " ```"] # [doc = ""] # [doc = " ### Heap into Vec"] # [doc = ""] # [doc = " For `TinyVec::Heap(vec_data)`,"] # [doc = "   `.into()` will take `vec_data` without heap reallocation."] # [doc = ""] # [doc = " ```"] # [doc = " use core::{"] # [doc = "   any::type_name_of_val as type_of, mem::size_of_val as mem_size_of,"] # [doc = " };"] # [doc = " use tinyvec::TinyVec;"] # [doc = ""] # [doc = " const fn from_heap<T: Default>(owned: Vec<T>) -> TinyVec<[T; 1]> {"] # [doc = "   TinyVec::Heap(owned)"] # [doc = " }"] # [doc = ""] # [doc = " let v = from_heap(vec![0u8; 128]);"] # [doc = " assert_eq!(v.len(), 128);"] # [doc = " assert_eq!(mem_size_of(&v), 24);"] # [doc = " assert!(type_of(&v).ends_with(\"TinyVec<[u8; 1]>\"));"] # [doc = ""] # [doc = " let vec: Vec<_> = v.into();"] # [doc = " assert_eq!(mem_size_of(&vec), 24);"] # [doc = " assert!(type_of(&vec).ends_with(\"Vec<u8>\"));"] # [doc = " ```"] # [inline] fn into (self) -> Vec < A :: Item > { match self { Self :: Heap (inner) => inner , Self :: Inline (mut inner) => inner . drain_to_vec () , } } }
    };
}

impl_153!()