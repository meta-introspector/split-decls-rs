macro_rules! DeprecatedSinceKind {
    () => {
        # [derive (Debug , Clone)] pub enum DeprecatedSinceKind { InEffect , InFuture , InVersion (String) , }
    };
}

DeprecatedSinceKind!();