macro_rules! from_raw {
    () => {
        fn from_raw < T > (val : * mut T) -> Option < Box < T > > { if val . is_null () { None } else { Some (unsafe { Box :: from_raw (val) }) } }
    };
}

from_raw!();