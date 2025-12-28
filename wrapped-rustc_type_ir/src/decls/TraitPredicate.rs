macro_rules! deps {
    () => {
        TraitRef!();
        PredicatePolarity!();
        Interner!();
    };
}

macro_rules! TraitPredicate {
    () => {
        deps!();
        # [derive_where (Clone , Copy , Hash , PartialEq ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct TraitPredicate < I : Interner > { pub trait_ref : TraitRef < I > , # [doc = " If polarity is Positive: we are proving that the trait is implemented."] # [doc = ""] # [doc = " If polarity is Negative: we are proving that a negative impl of this trait"] # [doc = " exists. (Note that coherence also checks whether negative impls of supertraits"] # [doc = " exist via a series of predicates.)"] pub polarity : PredicatePolarity , }
    };
}

TraitPredicate!()