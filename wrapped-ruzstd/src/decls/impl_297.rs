macro_rules! deps {
    () => {
        SequencesHeader!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl Default for SequencesHeader { fn default () -> Self { Self :: new () } }
    };
}

impl_297!();