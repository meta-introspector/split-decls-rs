macro_rules! deps {
    () => {
        Assert!();
    };
}

macro_rules! OutputAssert {
    () => {
        deps!();
        # [doc = " Assert the state of a [`Command`]'s [`Output`]."] # [doc = ""] # [doc = " Create an `OutputAssert` through the [`Command::assert`]."] # [doc = ""] # [doc = " [`Output`]: std::process::Output"] pub struct OutputAssert { output : std :: process :: Output , config : crate :: Assert , }
    };
}

OutputAssert!();