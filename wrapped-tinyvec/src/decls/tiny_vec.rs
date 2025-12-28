macro_rules! deps {
    () => {
        TinyVec!();
        TinyVecConstructor!();
    };
}

macro_rules! tiny_vec {
    () => {
        deps!();
        # [doc = " Helper to make a `TinyVec`."] # [doc = ""] # [doc = " You specify the backing array type, and optionally give all the elements you"] # [doc = " want to initially place into the array."] # [doc = ""] # [doc = " ```rust"] # [doc = " use tinyvec::*;"] # [doc = ""] # [doc = " // The backing array type can be specified in the macro call"] # [doc = " let empty_tv = tiny_vec!([u8; 16]);"] # [doc = " let some_ints = tiny_vec!([i32; 4] => 1, 2, 3);"] # [doc = " let many_ints = tiny_vec!([i32; 4] => 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);"] # [doc = ""] # [doc = " // Or left to inference"] # [doc = " let empty_tv: TinyVec<[u8; 16]> = tiny_vec!();"] # [doc = " let some_ints: TinyVec<[i32; 4]> = tiny_vec!(1, 2, 3);"] # [doc = " let many_ints: TinyVec<[i32; 4]> = tiny_vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);"] # [doc = " ```"] # [macro_export] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] macro_rules ! tiny_vec { ($ array_type : ty => $ ($ elem : expr) ,* $ (,) ?) => { { const INVOKED_ELEM_COUNT : usize = 0 $ (+ { let _ = stringify ! ($ elem) ; 1 }) *; match $ crate :: TinyVec :: constructor_for_capacity (INVOKED_ELEM_COUNT) { $ crate :: TinyVecConstructor :: Inline (f) => { f ($ crate :: array_vec ! ($ array_type => $ ($ elem) ,*)) } $ crate :: TinyVecConstructor :: Heap (f) => { f (vec ! ($ ($ elem) ,*)) } } } } ; ($ array_type : ty) => { $ crate :: TinyVec ::<$ array_type >:: default () } ; ($ ($ elem : expr) ,*) => { $ crate :: tiny_vec ! (_ => $ ($ elem) ,*) } ; ($ elem : expr ; $ n : expr) => { $ crate :: TinyVec :: from ([$ elem ; $ n]) } ; () => { $ crate :: tiny_vec ! (_) } ; }
    };
}

tiny_vec!()