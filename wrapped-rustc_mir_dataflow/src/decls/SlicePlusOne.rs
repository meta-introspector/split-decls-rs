macro_rules! SlicePlusOne {
    () => {
        struct SlicePlusOne < 'a , T > { slice : & 'a [T] , last : Option < T > , }
    };
}

SlicePlusOne!();