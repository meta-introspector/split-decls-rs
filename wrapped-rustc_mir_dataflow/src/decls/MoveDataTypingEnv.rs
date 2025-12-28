macro_rules! deps {
    () => {
        MoveData!();
    };
}

macro_rules! MoveDataTypingEnv {
    () => {
        deps!();
        pub struct MoveDataTypingEnv < 'tcx > { pub move_data : MoveData < 'tcx > , pub typing_env : ty :: TypingEnv < 'tcx > , }
    };
}

MoveDataTypingEnv!();