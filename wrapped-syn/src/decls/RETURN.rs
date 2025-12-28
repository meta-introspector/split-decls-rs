macro_rules! deps {
    () => {
        Input!();
        Action!();
    };
}

macro_rules! RETURN {
    () => {
        deps!();
        static RETURN : [(Input , Action) ; 2] = [(CanBeginExpr , SetState (& INIT)) , (Otherwise , SetState (& POSTFIX)) ,] ;
    };
}

RETURN!()