macro_rules! deps {
    () => {
        Borrowed!();
        Owned!();
    };
}

macro_rules! sealed {
    () => {
        deps!();
        mod sealed { pub trait ShouldCountInner { } impl ShouldCountInner for super :: Borrowed { } impl ShouldCountInner for super :: Owned { } }
    };
}

sealed!();