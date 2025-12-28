macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! ThiserrorProvide {
    () => {
        deps!();
        # [doc (hidden)] pub trait ThiserrorProvide : Sealed { fn thiserror_provide < 'a > (& 'a self , request : & mut Request < 'a >) ; }
    };
}

ThiserrorProvide!()