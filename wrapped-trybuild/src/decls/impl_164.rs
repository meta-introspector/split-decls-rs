macro_rules! deps {
    () => {
        Outcome!();
        Project!();
        Result!();
        ExpandedTest!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl ExpandedTest { fn run (self , project : & Project) -> Result < Outcome > { match self . error { None => self . test . run (project , & self . name) , Some (error) => { let show_expected = false ; message :: begin_test (& self . test , show_expected) ; Err (error) } } } }
    };
}

impl_164!();