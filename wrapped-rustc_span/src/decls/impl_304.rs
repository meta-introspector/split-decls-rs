macro_rules! deps {
    () => {
        SourceFileLines!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl SourceFileLines { pub fn is_lines (& self) -> bool { matches ! (self , SourceFileLines :: Lines (_)) } }
    };
}

impl_304!();