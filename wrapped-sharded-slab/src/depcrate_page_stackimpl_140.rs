// Generated macro for impl_140 (impl)
macro_rules! Depcrate_page_stackimpl_140 {
() => {
// Module: crate::page::stack
// Provides: {"impl_140"}
// Dependencies: {}
impl < C : cfg :: Config > TransferStack < C > { pub (super) fn new () -> Self { Self { head : AtomicUsize :: new (super :: Addr :: < C > :: NULL) , _cfg : PhantomData , } } pub (super) fn pop_all (& self) -> Option < usize > { let val = self . head . swap (super :: Addr :: < C > :: NULL , Ordering :: Acquire) ; test_println ! ("-> pop {:#x}" , val) ; if val == super :: Addr :: < C > :: NULL { None } else { Some (val) } } fn push (& self , new_head : usize , before : impl Fn (usize)) { let mut next = self . head . load (Ordering :: Relaxed) ; loop { test_println ! ("-> next {:#x}" , next) ; before (next) ; match self . head . compare_exchange (next , new_head , Ordering :: Release , Ordering :: Relaxed) { Err (actual) => { test_println ! ("-> retry!") ; next = actual ; } Ok (_) => { test_println ! ("-> successful; next={:#x}" , next) ; return ; } } } } }
};
}
