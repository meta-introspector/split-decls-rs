macro_rules! Action {
    () => {
        # [derive (Debug , Clone)] enum Action < T : Unpin > { Next (T) , Wait (Duration) , }
    };
}

Action!();