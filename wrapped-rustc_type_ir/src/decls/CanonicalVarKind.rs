macro_rules! deps {
    () => {
        Const!();
        Interner!();
        Ty!();
        PlaceholderConst!();
        Region!();
    };
}

macro_rules! CanonicalVarKind {
    () => {
        deps!();
        # [doc = " Information about a canonical variable that is included with the"] # [doc = " canonical value. This is sufficient information for code to create"] # [doc = " a copy of the canonical value in some other inference context,"] # [doc = " with fresh inference variables replacing the canonical values."] # [derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum CanonicalVarKind < I : Interner > { # [doc = " General type variable `?T` that can be unified with arbitrary types."] # [doc = ""] # [doc = " We also store the index of the first type variable which is sub-unified"] # [doc = " with this one. If there is no inference variable related to this one,"] # [doc = " its `sub_root` just points to itself."] Ty { ui : UniverseIndex , sub_root : ty :: BoundVar } , # [doc = " Integral type variable `?I` (that can only be unified with integral types)."] Int , # [doc = " Floating-point type variable `?F` (that can only be unified with float types)."] Float , # [doc = " A \"placeholder\" that represents \"any type\"."] PlaceholderTy (I :: PlaceholderTy) , # [doc = " Region variable `'?R`."] Region (UniverseIndex) , # [doc = " A \"placeholder\" that represents \"any region\". Created when you"] # [doc = " are solving a goal like `for<'a> T: Foo<'a>` to represent the"] # [doc = " bound region `'a`."] PlaceholderRegion (I :: PlaceholderRegion) , # [doc = " Some kind of const inference variable."] Const (UniverseIndex) , # [doc = " A \"placeholder\" that represents \"any const\"."] PlaceholderConst (I :: PlaceholderConst) , }
    };
}

CanonicalVarKind!()