macro_rules! deps {
    () => {
        ThreadWaker!();
    };
}

macro_rules! from_raw {
    () => {
        deps!();
        unsafe fn from_raw (raw : * const ()) -> Arc < ThreadWaker > { Arc :: from_raw (raw as * const ThreadWaker) }
    };
}

from_raw!();