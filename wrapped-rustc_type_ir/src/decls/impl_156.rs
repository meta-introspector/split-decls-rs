macro_rules! deps {
    () => {
        PathKind!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl PathsToNested { # [doc = " The implementation of this function is kind of ugly. We check whether"] # [doc = " there currently exist 'weaker' paths in the set, if so we upgrade these"] # [doc = " paths to at least `path`."] # [must_use] fn extend_with (mut self , path : PathKind) -> Self { match path { PathKind :: Inductive => { if self . intersects (PathsToNested :: EMPTY) { self . remove (PathsToNested :: EMPTY) ; self . insert (PathsToNested :: INDUCTIVE) ; } } PathKind :: Unknown => { if self . intersects (PathsToNested :: EMPTY | PathsToNested :: INDUCTIVE) { self . remove (PathsToNested :: EMPTY | PathsToNested :: INDUCTIVE) ; self . insert (PathsToNested :: UNKNOWN) ; } } PathKind :: Coinductive => { if self . intersects (PathsToNested :: EMPTY | PathsToNested :: INDUCTIVE | PathsToNested :: UNKNOWN ,) { self . remove (PathsToNested :: EMPTY | PathsToNested :: INDUCTIVE | PathsToNested :: UNKNOWN ,) ; self . insert (PathsToNested :: COINDUCTIVE) ; } } PathKind :: ForcedAmbiguity => { if self . intersects (PathsToNested :: EMPTY | PathsToNested :: INDUCTIVE | PathsToNested :: UNKNOWN | PathsToNested :: COINDUCTIVE ,) { self . remove (PathsToNested :: EMPTY | PathsToNested :: INDUCTIVE | PathsToNested :: UNKNOWN | PathsToNested :: COINDUCTIVE ,) ; self . insert (PathsToNested :: FORCED_AMBIGUITY) ; } } } self } # [must_use] fn extend_with_paths (self , path : PathsToNested) -> Self { let mut new = PathsToNested :: empty () ; for p in path . iter_paths () { new |= self . extend_with (p) ; } new } fn iter_paths (self) -> impl Iterator < Item = PathKind > { let (PathKind :: Inductive | PathKind :: Unknown | PathKind :: Coinductive | PathKind :: ForcedAmbiguity) ; [PathKind :: Inductive , PathKind :: Unknown , PathKind :: Coinductive , PathKind :: ForcedAmbiguity] . into_iter () . filter (move | & p | self . contains (p . into ())) } }
    };
}

impl_156!();