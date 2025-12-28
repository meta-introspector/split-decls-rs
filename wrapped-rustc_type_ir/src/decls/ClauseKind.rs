macro_rules! deps {
    () => {
        OutlivesPredicate!();
        Term!();
        Region!();
        TraitPredicate!();
        Ty!();
        HostEffectPredicate!();
        Const!();
        ProjectionPredicate!();
        Interner!();
    };
}

macro_rules! ClauseKind {
    () => {
        deps!();
        # [doc = " A clause is something that can appear in where bounds or be inferred"] # [doc = " by implied bounds."] # [derive_where (Clone , Copy , Hash , PartialEq ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub enum ClauseKind < I : Interner > { # [doc = " Corresponds to `where Foo: Bar<A, B, C>`. `Foo` here would be"] # [doc = " the `Self` type of the trait reference and `A`, `B`, and `C`"] # [doc = " would be the type parameters."] Trait (ty :: TraitPredicate < I >) , # [doc = " `where 'a: 'r`"] RegionOutlives (ty :: OutlivesPredicate < I , I :: Region >) , # [doc = " `where T: 'r`"] TypeOutlives (ty :: OutlivesPredicate < I , I :: Ty >) , # [doc = " `where <T as TraitRef>::Name == X`, approximately."] # [doc = " See the `ProjectionPredicate` struct for details."] Projection (ty :: ProjectionPredicate < I >) , # [doc = " Ensures that a const generic argument to a parameter `const N: u8`"] # [doc = " is of type `u8`."] ConstArgHasType (I :: Const , I :: Ty) , # [doc = " No syntax: `T` well-formed."] WellFormed (I :: Term) , # [doc = " Constant initializer must evaluate successfully."] ConstEvaluatable (I :: Const) , # [doc = " Enforces the constness of the predicate we're calling. Like a projection"] # [doc = " goal from a where clause, it's always going to be paired with a"] # [doc = " corresponding trait clause; this just enforces the *constness* of that"] # [doc = " implementation."] HostEffect (ty :: HostEffectPredicate < I >) , # [doc = " Support marking impl as unstable."] UnstableFeature (# [type_foldable (identity)] # [type_visitable (ignore)] I :: Symbol ,) , }
    };
}

ClauseKind!();