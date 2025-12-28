macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! AsDisplay {
    () => {
        deps!();
        # [doc (hidden)] pub trait AsDisplay < 'a > : Sealed { type Target : Display ; fn as_display (& 'a self) -> Self :: Target ; }
    };
}

AsDisplay!();