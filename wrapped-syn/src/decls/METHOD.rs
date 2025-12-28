macro_rules! deps {
    () => {
        Input!();
        Action!();
    };
}

macro_rules! METHOD {
    () => {
        deps!();
        static METHOD : [(Input , Action) ; 1] = [(ExpectTurbofish , SetState (& POSTFIX))] ;
    };
}

METHOD!()