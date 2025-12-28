macro_rules! deps {
    () => {
        NativeEndian!();
    };
}

macro_rules! macro_155 {
    () => {
        deps!();
        module ! (native_endian , NativeEndian , "native-endian") ;
    };
}

macro_155!();