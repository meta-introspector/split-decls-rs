macro_rules! Align8 {
    () => {
        # [repr (C , align (8))] pub (crate) struct Align8 < T > (pub (crate) T) ;
    };
}

Align8!();