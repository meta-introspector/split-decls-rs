macro_rules! OrderedF64 {
    () => {
        # [derive (PartialEq , Default , Copy , Clone)] pub struct OrderedF64 (f64) ;
    };
}

OrderedF64!();