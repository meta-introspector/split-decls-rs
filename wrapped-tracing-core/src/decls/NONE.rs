macro_rules! deps {
    () => {
        Kind!();
        Dispatch!();
    };
}

macro_rules! NONE {
    () => {
        deps!();
        static NONE : Dispatch = Dispatch { subscriber : Kind :: Global (& NO_SUBSCRIBER) , } ;
    };
}

NONE!()