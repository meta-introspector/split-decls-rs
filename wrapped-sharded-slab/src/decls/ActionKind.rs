macro_rules! deps {
    () => {
        VacantEntry!();
    };
}

macro_rules! ActionKind {
    () => {
        deps!();
        # [derive (Debug , Clone)] enum ActionKind { Insert , VacantEntry , RemoveRandom (usize) , RemoveExistent (usize) , TakeRandom (usize) , TakeExistent (usize) , GetRandom (usize) , GetExistent (usize) , }
    };
}

ActionKind!();