macro_rules! Align64 {
    () => {
        # [repr (align (64))] struct Align64 < T > (T) ;
    };
}

Align64!()