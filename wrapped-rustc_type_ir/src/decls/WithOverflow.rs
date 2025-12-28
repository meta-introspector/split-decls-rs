macro_rules! deps {
    () => {
        NestedGoals!();
        Cx!();
    };
}

macro_rules! WithOverflow {
    () => {
        deps!();
        struct WithOverflow < X : Cx > { nested_goals : NestedGoals < X > , result : X :: Tracked < X :: Result > , }
    };
}

WithOverflow!()