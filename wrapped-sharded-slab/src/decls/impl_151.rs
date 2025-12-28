macro_rules! deps {
    () => {
        Shared!();
        Slot!();
        Config!();
        Local!();
        Shard!();
        Tid!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < T , C > Shard < T , C > where C : cfg :: Config , { # [inline (always)] pub (crate) fn with_slot < 'a , U > (& 'a self , idx : usize , f : impl FnOnce (& 'a page :: Slot < T , C >) -> Option < U > ,) -> Option < U > { debug_assert_eq_in_drop ! (Tid ::< C >:: from_packed (idx) . as_usize () , self . tid) ; let (addr , page_index) = page :: indices :: < C > (idx) ; test_println ! ("-> {:?}" , addr) ; if page_index >= self . shared . len () { return None ; } self . shared [page_index] . with_slot (addr , f) } pub (crate) fn new (tid : usize) -> Self { let mut total_sz = 0 ; let shared = (0 .. C :: MAX_PAGES) . map (| page_num | { let sz = C :: page_size (page_num) ; let prev_sz = total_sz ; total_sz += sz ; page :: Shared :: new (sz , prev_sz) }) . collect () ; let local = (0 .. C :: MAX_PAGES) . map (| _ | page :: Local :: new ()) . collect () ; Self { tid , local , shared } } }
    };
}

impl_151!()