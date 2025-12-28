macro_rules! deps {
    () => {
        Iter!();
        Error!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < 'a > Iterator for Iter < 'a > { type Item = Error ; fn next (& mut self) -> Option < Self :: Item > { Some (Error { messages : vec ! [self . messages . next () ?. clone ()] , }) } }
    };
}

impl_194!()