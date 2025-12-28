macro_rules! deps {
    () => {
        TypeError!();
        Interner!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < I : Interner > TypeError < I > { pub fn involves_regions (self) -> bool { match self { TypeError :: RegionsDoesNotOutlive (_ , _) | TypeError :: RegionsInsufficientlyPolymorphic (_ , _) | TypeError :: RegionsPlaceholderMismatch => true , _ => false , } } pub fn must_include_note (self) -> bool { use self :: TypeError :: * ; match self { CyclicTy (_) | CyclicConst (_) | SafetyMismatch (_) | PolarityMismatch (_) | Mismatch | AbiMismatch (_) | ArraySize (_) | ArgumentSorts (..) | Sorts (_) | VariadicMismatch (_) | TargetFeatureCast (_) => false , Mutability | ArgumentMutability (_) | TupleSize (_) | ArgCount | RegionsDoesNotOutlive (..) | RegionsInsufficientlyPolymorphic (..) | RegionsPlaceholderMismatch | Traits (_) | ProjectionMismatched (_) | ExistentialMismatch (_) | ConstMismatch (_) | ForceInlineCast | IntrinsicCast => true , } } }
    };
}

impl_36!()