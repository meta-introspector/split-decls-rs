macro_rules! deps {
    () => {
        Action!();
        Input!();
    };
}

macro_rules! IF_THEN {
    () => {
        deps!();
        static IF_THEN : [(Input , Action) ; 2] = [(Keyword ("else") , SetState (& IF_ELSE)) , (Otherwise , DecDepth)] ;
    };
}

IF_THEN!()