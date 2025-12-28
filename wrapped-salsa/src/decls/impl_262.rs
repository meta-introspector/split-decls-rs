macro_rules! deps {
    () => {
        Revision!();
        OptionalAtomicRevision!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl OptionalAtomicRevision { pub (crate) fn new (revision : Option < Revision >) -> Self { Self { data : AtomicUsize :: new (revision . map_or (0 , | r | r . as_usize ())) , } } pub (crate) fn load (& self) -> Option < Revision > { Revision :: from_opt (self . data . load (Ordering :: Acquire)) } pub (crate) fn swap (& self , val : Option < Revision >) -> Option < Revision > { Revision :: from_opt (self . data . swap (val . map_or (0 , | r | r . as_usize ()) , Ordering :: AcqRel) ,) } pub (crate) fn compare_exchange (& self , current : Option < Revision > , new : Option < Revision > ,) -> Result < Option < Revision > , Option < Revision > > { self . data . compare_exchange (current . map_or (0 , | r | r . as_usize ()) , new . map_or (0 , | r | r . as_usize ()) , Ordering :: AcqRel , Ordering :: Acquire ,) . map (Revision :: from_opt) . map_err (Revision :: from_opt) } }
    };
}

impl_262!();