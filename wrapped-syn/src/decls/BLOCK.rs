macro_rules! deps {
    () => {
        Action!();
        Input!();
    };
}

macro_rules! BLOCK {
    () => {
        deps!();
        static BLOCK : [(Input , Action) ; 1] = [(ConsumeBrace , SetState (& POSTFIX))] ;
    };
}

BLOCK!();