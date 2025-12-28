macro_rules! Align64 {
    () => {
        # [repr (C , align (64))] pub (crate) struct Align64 < T > (pub (crate) T) ;
    };
}

Align64!();