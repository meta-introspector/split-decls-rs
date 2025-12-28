macro_rules! deps {
    () => {
        Weak!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < F > Weak < F > { pub (crate) const fn new (name : & 'static str) -> Self { Self { name , addr : AtomicPtr :: new (INVALID) , _marker : marker :: PhantomData , } } pub (crate) fn get (& self) -> Option < F > { assert_eq ! (mem :: size_of ::< F > () , mem :: size_of ::< usize > ()) ; unsafe { match self . addr . load (Ordering :: Relaxed) { INVALID => self . initialize () , NULL => None , addr => { let func = mem :: transmute_copy :: < * mut c_void , F > (& addr) ; atomic :: fence (Ordering :: Acquire) ; Some (func) } } } } # [cold] unsafe fn initialize (& self) -> Option < F > { let val = fetch (self . name) ; self . addr . store (val , Ordering :: Release) ; match val { NULL => None , addr => Some (mem :: transmute_copy :: < * mut c_void , F > (& addr)) , } } }
    };
}

impl_56!()