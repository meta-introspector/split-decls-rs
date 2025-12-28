macro_rules! deps {
    () => {
        NestedGoals!();
        Cx!();
    };
}

macro_rules! Success {
    () => {
        deps!();
        struct Success < X : Cx > { required_depth : usize , nested_goals : NestedGoals < X > , result : X :: Tracked < X :: Result > , }
    };
}

Success!()