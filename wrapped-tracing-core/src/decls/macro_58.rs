macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! macro_58 {
    () => {
        deps!();
        # [cfg (feature = "std")] std :: thread_local ! { static CURRENT_STATE : State = const { State { default : RefCell :: new (None) , can_enter : Cell :: new (true) , } } ; }
    };
}

macro_58!();