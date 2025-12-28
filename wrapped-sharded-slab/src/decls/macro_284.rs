macro_rules! deps {
    () => {
        DefaultConfig!();
        CustomConfig!();
    };
}

macro_rules! macro_284 {
    () => {
        deps!();
        proptest ! { # [test] fn default_config (actions in prop :: collection :: vec (action_strategy () , ACTIONS)) { run ::< DefaultConfig > (actions) ?; } # [test] fn custom_config (actions in prop :: collection :: vec (action_strategy () , ACTIONS)) { run ::< CustomConfig > (actions) ?; } }
    };
}

macro_284!();