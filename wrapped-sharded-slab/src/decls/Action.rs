macro_rules! deps {
    () => {
        ActionKind!();
    };
}

macro_rules! Action {
    () => {
        deps!();
        # [derive (Debug , Clone)] struct Action { tid : usize , kind : ActionKind , }
    };
}

Action!();