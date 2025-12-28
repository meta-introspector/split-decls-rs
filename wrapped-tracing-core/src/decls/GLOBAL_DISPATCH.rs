macro_rules! deps {
    () => {
        Dispatch!();
        Kind!();
    };
}

macro_rules! GLOBAL_DISPATCH {
    () => {
        deps!();
        static mut GLOBAL_DISPATCH : Dispatch = Dispatch { subscriber : Kind :: Global (& NO_SUBSCRIBER) , } ;
    };
}

GLOBAL_DISPATCH!()