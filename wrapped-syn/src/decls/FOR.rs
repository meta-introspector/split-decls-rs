macro_rules! deps {
    () => {
        Input!();
        Action!();
    };
}

macro_rules! FOR {
    () => {
        deps!();
        static FOR : [(Input , Action) ; 2] = [(Punct ("<") , SetState (& CLOSURE)) , (Otherwise , SetState (& PATTERN)) ,] ;
    };
}

FOR!();