macro_rules! StrLitKind {
    () => {
        # [derive (Copy , Clone)] enum StrLitKind { Normal , Raw (usize) , }
    };
}

StrLitKind!();