macro_rules! deps {
    () => {
        MakeVisitor!();
        Messages!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < V > Messages < V > { # [doc = " Returns a new [`MakeVisitor`] implementation that will wrap `inner` so"] # [doc = " that any strings named `message` are formatted using `fmt::Display`."] # [doc = ""] # [doc = " [`MakeVisitor`]: super::MakeVisitor"] pub fn new (inner : V) -> Self { Messages (inner) } }
    };
}

impl_24!()