macro_rules! deps {
    () => {
        Input!();
        Action!();
    };
}

macro_rules! RAW {
    () => {
        deps!();
        static RAW : [(Input , Action) ; 3] = [(Keyword ("const") , SetState (& INIT)) , (Keyword ("mut") , SetState (& INIT)) , (Otherwise , SetState (& POSTFIX)) ,] ;
    };
}

RAW!()