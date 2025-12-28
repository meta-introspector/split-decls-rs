macro_rules! deps {
    () => {
        Config!();
        Slot!();
        Shard!();
        Tid!();
        Clear!();
        Local!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < T , C > Shard < T , C > where T : Clear + Default , C : cfg :: Config , { pub (crate) fn init_with < U > (& self , mut init : impl FnMut (usize , & page :: Slot < T , C >) -> Option < U > ,) -> Option < U > { for (page_idx , page) in self . shared . iter () . enumerate () { let local = self . local (page_idx) ; test_println ! ("-> page {}; {:?}; {:?}" , page_idx , local , page) ; if let Some (res) = page . init_with (local , & mut init) { return Some (res) ; } } None } pub (crate) fn mark_clear_local (& self , idx : usize) -> bool { debug_assert_eq_in_drop ! (Tid ::< C >:: from_packed (idx) . as_usize () , self . tid) ; let (addr , page_index) = page :: indices :: < C > (idx) ; if page_index >= self . shared . len () { return false ; } self . shared [page_index] . mark_clear (addr , C :: unpack_gen (idx) , self . local (page_index)) } pub (crate) fn mark_clear_remote (& self , idx : usize) -> bool { debug_assert_eq_in_drop ! (Tid ::< C >:: from_packed (idx) . as_usize () , self . tid) ; let (addr , page_index) = page :: indices :: < C > (idx) ; if page_index >= self . shared . len () { return false ; } let shared = & self . shared [page_index] ; shared . mark_clear (addr , C :: unpack_gen (idx) , shared . free_list ()) } pub (crate) fn clear_after_release (& self , idx : usize) { crate :: sync :: atomic :: fence (crate :: sync :: atomic :: Ordering :: Acquire) ; let tid = Tid :: < C > :: current () . as_usize () ; test_println ! ("-> clear_after_release; self.tid={:?}; current.tid={:?};" , tid , self . tid) ; if tid == self . tid { self . clear_local (idx) ; } else { self . clear_remote (idx) ; } } fn clear_local (& self , idx : usize) -> bool { debug_assert_eq_in_drop ! (Tid ::< C >:: from_packed (idx) . as_usize () , self . tid) ; let (addr , page_index) = page :: indices :: < C > (idx) ; if page_index >= self . shared . len () { return false ; } self . shared [page_index] . clear (addr , C :: unpack_gen (idx) , self . local (page_index)) } fn clear_remote (& self , idx : usize) -> bool { debug_assert_eq_in_drop ! (Tid ::< C >:: from_packed (idx) . as_usize () , self . tid) ; let (addr , page_index) = page :: indices :: < C > (idx) ; if page_index >= self . shared . len () { return false ; } let shared = & self . shared [page_index] ; shared . clear (addr , C :: unpack_gen (idx) , shared . free_list ()) } # [inline (always)] fn local (& self , i : usize) -> & page :: Local { # [cfg (debug_assertions)] debug_assert_eq_in_drop ! (Tid ::< C >:: current () . as_usize () , self . tid , "tried to access local data from another thread!") ; & self . local [i] } }
    };
}

impl_153!()