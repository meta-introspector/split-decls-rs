macro_rules! deps {
    () => {
        Interner!();
        QueryResult!();
        Probe!();
        CandidateSource!();
    };
}

macro_rules! ProbeKind {
    () => {
        deps!();
        # [doc = " What kind of probe we're in. In case the probe represents a candidate, or"] # [doc = " the final result of the current goal - via [ProbeKind::Root] - we also"] # [doc = " store the [QueryResult]."] # [derive_where (Clone , Copy , PartialEq , Eq , Hash , Debug ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic)] pub enum ProbeKind < I : Interner > { # [doc = " The root inference context while proving a goal."] Root { result : QueryResult < I > } , # [doc = " Probe entered when normalizing the self ty during candidate assembly"] NormalizedSelfTyAssembly , # [doc = " A candidate for proving a trait or alias-relate goal."] TraitCandidate { source : CandidateSource < I > , result : QueryResult < I > } , # [doc = " Used in the probe that wraps normalizing the non-self type for the unsize"] # [doc = " trait, which is also structurally matched on."] UnsizeAssembly , # [doc = " Used to do a probe to find out what projection type(s) match a given"] # [doc = " alias bound or projection predicate. For trait upcasting, this is used"] # [doc = " to prove that the source type upholds all of the target type's object"] # [doc = " bounds. For object type bounds, this is used when eagerly replacing"] # [doc = " supertrait aliases."] ProjectionCompatibility , # [doc = " Looking for param-env candidates that satisfy the trait ref for a projection."] ShadowedEnvProbing , # [doc = " Try to unify an opaque type with an existing key in the storage."] OpaqueTypeStorageLookup { result : QueryResult < I > } , # [doc = " Checking that a rigid alias is well-formed."] RigidAlias { result : QueryResult < I > } , }
    };
}

ProbeKind!()