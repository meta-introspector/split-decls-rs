macro_rules! deps {
    () => {
        AssembleCandidatesFrom!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl AssembleCandidatesFrom { fn should_assemble_impl_candidates (& self) -> bool { match self { AssembleCandidatesFrom :: All => true , AssembleCandidatesFrom :: EnvAndBounds => false , } } }
    };
}

impl_57!()