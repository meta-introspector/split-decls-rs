macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! Computed {
    () => {
        deps!();
        # [doc = "\nA `Stream` that accepts values for any lifetime.\n"] # [repr (transparent)] struct Computed < S : ? Sized > (S) ;
    };
}

Computed!();