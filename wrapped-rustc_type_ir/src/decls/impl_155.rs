macro_rules! deps {
    () => {
        PathKind!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl From < PathKind > for PathsToNested { fn from (path : PathKind) -> PathsToNested { match path { PathKind :: Inductive => PathsToNested :: INDUCTIVE , PathKind :: Unknown => PathsToNested :: UNKNOWN , PathKind :: Coinductive => PathsToNested :: COINDUCTIVE , PathKind :: ForcedAmbiguity => PathsToNested :: FORCED_AMBIGUITY , } } }
    };
}

impl_155!()