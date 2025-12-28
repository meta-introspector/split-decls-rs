macro_rules! deps {
    () => {
        Error!();
        IntoIter!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl Iterator for IntoIter { type Item = Error ; fn next (& mut self) -> Option < Self :: Item > { Some (Error { messages : vec ! [self . messages . next () ?] , }) } }
    };
}

impl_191!()