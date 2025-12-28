macro_rules! deps {
    () => {
        IngredientIndex!();
        IdentityMap!();
        IdentityHash!();
        Identity!();
        Id!();
        DisambiguatorMap!();
        Disambiguator!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: * ; # [test] fn disambiguate_map_works () { let mut d = DisambiguatorMap :: default () ; let h1 = IdentityHash { ingredient_index : IngredientIndex :: new (0) , hash : 0 , } ; let h2 = IdentityHash { ingredient_index : IngredientIndex :: new (1) , hash : 0 , } ; let h3 = IdentityHash { ingredient_index : IngredientIndex :: new (0) , hash : 1 , } ; let h4 = IdentityHash { ingredient_index : IngredientIndex :: new (1) , hash : 1 , } ; assert_eq ! (d . disambiguate (h1) , Disambiguator (0)) ; assert_eq ! (d . disambiguate (h1) , Disambiguator (1)) ; assert_eq ! (d . disambiguate (h2) , Disambiguator (0)) ; assert_eq ! (d . disambiguate (h2) , Disambiguator (1)) ; assert_eq ! (d . disambiguate (h3) , Disambiguator (0)) ; assert_eq ! (d . disambiguate (h3) , Disambiguator (1)) ; assert_eq ! (d . disambiguate (h4) , Disambiguator (0)) ; assert_eq ! (d . disambiguate (h4) , Disambiguator (1)) ; } # [test] fn identity_map_works () { let mut d = IdentityMap :: default () ; let i1 = Identity { ingredient_index : IngredientIndex :: new (0) , hash : 0 , disambiguator : Disambiguator (0) , } ; let i2 = Identity { ingredient_index : IngredientIndex :: new (1) , hash : 0 , disambiguator : Disambiguator (0) , } ; let i3 = Identity { ingredient_index : IngredientIndex :: new (0) , hash : 1 , disambiguator : Disambiguator (0) , } ; let i4 = Identity { ingredient_index : IngredientIndex :: new (1) , hash : 1 , disambiguator : Disambiguator (0) , } ; let i5 = Identity { ingredient_index : IngredientIndex :: new (0) , hash : 0 , disambiguator : Disambiguator (1) , } ; let i6 = Identity { ingredient_index : IngredientIndex :: new (1) , hash : 0 , disambiguator : Disambiguator (1) , } ; let i7 = Identity { ingredient_index : IngredientIndex :: new (0) , hash : 1 , disambiguator : Disambiguator (1) , } ; let i8 = Identity { ingredient_index : IngredientIndex :: new (1) , hash : 1 , disambiguator : Disambiguator (1) , } ; unsafe { assert_eq ! (d . insert (i1 , Id :: from_index (0)) , None) ; assert_eq ! (d . insert (i2 , Id :: from_index (1)) , None) ; assert_eq ! (d . insert (i3 , Id :: from_index (2)) , None) ; assert_eq ! (d . insert (i4 , Id :: from_index (3)) , None) ; assert_eq ! (d . insert (i5 , Id :: from_index (4)) , None) ; assert_eq ! (d . insert (i6 , Id :: from_index (5)) , None) ; assert_eq ! (d . insert (i7 , Id :: from_index (6)) , None) ; assert_eq ! (d . insert (i8 , Id :: from_index (7)) , None) ; assert_eq ! (d . reuse (& i1) , Some (Id :: from_index (0))) ; assert_eq ! (d . reuse (& i2) , Some (Id :: from_index (1))) ; assert_eq ! (d . reuse (& i3) , Some (Id :: from_index (2))) ; assert_eq ! (d . reuse (& i4) , Some (Id :: from_index (3))) ; assert_eq ! (d . reuse (& i5) , Some (Id :: from_index (4))) ; assert_eq ! (d . reuse (& i6) , Some (Id :: from_index (5))) ; assert_eq ! (d . reuse (& i7) , Some (Id :: from_index (6))) ; assert_eq ! (d . reuse (& i8) , Some (Id :: from_index (7))) ; } ; } }
    };
}

tests!()