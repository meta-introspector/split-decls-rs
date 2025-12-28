macro_rules! deps {
    () => {
        Action!();
        Input!();
    };
}

macro_rules! CONTINUE {
    () => {
        deps!();
        static CONTINUE : [(Input , Action) ; 2] = [(ConsumeLifetime , SetState (& POSTFIX)) , (Otherwise , SetState (& POSTFIX)) ,] ;
    };
}

CONTINUE!()