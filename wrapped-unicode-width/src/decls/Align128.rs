macro_rules! Align128 {
    () => {
        # [repr (align (128))] struct Align128 < T > (T) ;
    };
}

Align128!()