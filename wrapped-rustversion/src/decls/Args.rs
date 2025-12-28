macro_rules! deps {
    () => {
        Then!();
        Expr!();
    };
}

macro_rules! Args {
    () => {
        deps!();
        pub struct Args { pub condition : Expr , pub then : Then , }
    };
}

Args!();