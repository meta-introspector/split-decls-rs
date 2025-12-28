macro_rules! deps {
    () => {
        DirRootInner!();
    };
}

macro_rules! DirRoot {
    () => {
        deps!();
        # [doc = " Working directory for tests"] # [derive (Debug)] pub struct DirRoot (DirRootInner) ;
    };
}

DirRoot!()