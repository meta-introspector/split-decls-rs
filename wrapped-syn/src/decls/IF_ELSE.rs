macro_rules! deps {
    () => {
        Input!();
        Action!();
    };
}

macro_rules! IF_ELSE {
    () => {
        deps!();
        static IF_ELSE : [(Input , Action) ; 2] = [(Keyword ("if") , SetState (& INIT)) , (ConsumeBrace , DecDepth)] ;
    };
}

IF_ELSE!();