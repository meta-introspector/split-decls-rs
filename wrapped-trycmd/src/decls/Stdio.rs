macro_rules! Stdio {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] enum Stdio { Stdout , Stderr , }
    };
}

Stdio!();