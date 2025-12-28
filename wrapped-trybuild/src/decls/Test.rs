macro_rules! deps {
    () => {
        Expected!();
    };
}

macro_rules! Test {
    () => {
        deps!();
        # [derive (Clone , Debug)] struct Test { path : PathBuf , expected : Expected , }
    };
}

Test!();