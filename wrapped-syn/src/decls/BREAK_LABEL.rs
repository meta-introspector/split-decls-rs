macro_rules! deps {
    () => {
        Input!();
        Action!();
    };
}

macro_rules! BREAK_LABEL {
    () => {
        deps!();
        static BREAK_LABEL : [(Input , Action) ; 2] = [(ConsumeLifetime , SetState (& BREAK_VALUE)) , (Otherwise , SetState (& BREAK_VALUE)) ,] ;
    };
}

BREAK_LABEL!();