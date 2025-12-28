macro_rules! deps {
    () => {
        Dispatch!();
        Kind!();
    };
}

macro_rules! NONE {
    () => {
        deps!();
        static NONE : Dispatch = Dispatch { subscriber : Kind :: Global (& NO_SUBSCRIBER) , } ;
    };
}

NONE!();