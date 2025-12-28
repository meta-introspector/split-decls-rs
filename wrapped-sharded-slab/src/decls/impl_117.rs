macro_rules! deps {
    () => {
        TransferStack!();
        Config!();
        Addr!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < C : cfg :: Config > TransferStack < C > { pub (super) fn new () -> Self { Self { head : AtomicUsize :: new (super :: Addr :: < C > :: NULL) , _cfg : PhantomData , } } pub (super) fn pop_all (& self) -> Option < usize > { let val = self . head . swap (super :: Addr :: < C > :: NULL , Ordering :: Acquire) ; test_println ! ("-> pop {:#x}" , val) ; if val == super :: Addr :: < C > :: NULL { None } else { Some (val) } } fn push (& self , new_head : usize , before : impl Fn (usize)) { let mut next = self . head . load (Ordering :: Relaxed) ; loop { test_println ! ("-> next {:#x}" , next) ; before (next) ; match self . head . compare_exchange (next , new_head , Ordering :: Release , Ordering :: Relaxed) { Err (actual) => { test_println ! ("-> retry!") ; next = actual ; } Ok (_) => { test_println ! ("-> successful; next={:#x}" , next) ; return ; } } } } }
    };
}

impl_117!();