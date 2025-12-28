macro_rules! deps {
    () => {
        Input!();
    };
}

macro_rules! Action {
    () => {
        deps!();
        enum Action { SetState (& 'static [(Input , Action)]) , IncDepth , DecDepth , Finish , }
    };
}

Action!()