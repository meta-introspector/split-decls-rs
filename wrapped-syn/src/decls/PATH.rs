macro_rules! deps {
    () => {
        Action!();
        Input!();
    };
}

macro_rules! PATH {
    () => {
        deps!();
        static PATH : [(Input , Action) ; 4] = [(Punct ("!=") , SetState (& INIT)) , (Punct ("!") , SetState (& INIT)) , (ConsumeNestedBrace , SetState (& IF_THEN)) , (Otherwise , SetState (& POSTFIX)) ,] ;
    };
}

PATH!()