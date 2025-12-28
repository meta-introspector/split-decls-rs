macro_rules! deps {
    () => {
        Action!();
    };
}

macro_rules! macro_277 {
    () => {
        deps!();
        prop_compose ! { fn action_strategy () (tid in THREADS , kind in action_kind_strategy ()) -> Action { Action { tid , kind } } }
    };
}

macro_277!();