macro_rules! deps {
    () => {
        UnevaluatedConst!();
    };
}

macro_rules! AliasTermKind {
    () => {
        deps!();
        # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [cfg_attr (feature = "nightly" , derive (Encodable , Decodable , HashStable_NoContext))] pub enum AliasTermKind { # [doc = " A projection `<Type as Trait>::AssocType`."] # [doc = " Can get normalized away if monomorphic enough."] ProjectionTy , # [doc = " An associated type in an inherent `impl`"] InherentTy , # [doc = " An opaque type (usually from `impl Trait` in type aliases or function return types)"] # [doc = " Can only be normalized away in PostAnalysis mode or its defining scope."] OpaqueTy , # [doc = " A free type alias that actually checks its trait bounds."] # [doc = " Currently only used if the type alias references opaque types."] # [doc = " Can always be normalized away."] FreeTy , # [doc = " An unevaluated anonymous constants."] UnevaluatedConst , # [doc = " An unevaluated const coming from an associated const."] ProjectionConst , # [doc = " A top level const item not part of a trait or impl."] FreeConst , # [doc = " An associated const in an inherent `impl`"] InherentConst , }
    };
}

AliasTermKind!()