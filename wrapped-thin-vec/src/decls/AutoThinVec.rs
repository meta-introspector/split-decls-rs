macro_rules! deps {
    () => {
        ThinVec!();
        AutoBuffer!();
    };
}

macro_rules! AutoThinVec {
    () => {
        deps!();
        # [doc (hidden)] # [cfg (feature = "gecko-ffi")] # [repr (C)] pub struct AutoThinVec < T , const N : usize > { inner : ThinVec < T > , buffer : AutoBuffer < T , N > , _pinned : std :: marker :: PhantomPinned , }
    };
}

AutoThinVec!()