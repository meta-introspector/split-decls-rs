macro_rules! MoveDataTypingEnv {
    () => {
        pub struct MoveDataTypingEnv < 'tcx > { pub move_data : MoveData < 'tcx > , pub typing_env : ty :: TypingEnv < 'tcx > , }
    };
}

MoveDataTypingEnv!()