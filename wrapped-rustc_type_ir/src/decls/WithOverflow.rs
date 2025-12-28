macro_rules! deps {
    () => {
        Cx!();
        NestedGoals!();
    };
}

macro_rules! WithOverflow {
    () => {
        deps!();
        struct WithOverflow < X : Cx > { nested_goals : NestedGoals < X > , result : X :: Tracked < X :: Result > , }
    };
}

WithOverflow!();