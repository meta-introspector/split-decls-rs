macro_rules! Expected {
    () => {
        # [derive (Copy , Clone , Debug)] enum Expected { Pass , CompileFail , }
    };
}

Expected!();