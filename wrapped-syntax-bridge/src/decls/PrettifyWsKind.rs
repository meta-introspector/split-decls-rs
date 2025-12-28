macro_rules! PrettifyWsKind {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum PrettifyWsKind { Space , Indent (usize) , Newline , }
    };
}

PrettifyWsKind!();