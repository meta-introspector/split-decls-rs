macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! Channel {
    () => {
        deps!();
        # [doc = " Release channel: \"dev\", \"nightly\", \"beta\", or \"stable\"."] # [derive (Debug , PartialEq , Eq , Copy , Clone)] pub struct Channel (Kind) ;
    };
}

Channel!()