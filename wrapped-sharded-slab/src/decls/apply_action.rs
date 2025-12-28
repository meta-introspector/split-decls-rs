macro_rules! deps {
    () => {
        ActionKind!();
        Config!();
        Active!();
        Slab!();
        VacantEntry!();
    };
}

macro_rules! apply_action {
    () => {
        deps!();
        fn apply_action < C : Config > (slab : & Arc < Slab < u32 , C > > , active : & mut Active , action : ActionKind ,) -> Result < () , TestCaseError > { match action { ActionKind :: Insert => { let value = active . next_value () ; let key = slab . insert (value) . expect ("unexpectedly exhausted slab") ; prop_assert_eq ! (used_bits ::< C > (key) , key) ; active . insert (key , value) ; } ActionKind :: VacantEntry => { let value = active . next_value () ; let entry = slab . vacant_entry () . expect ("unexpectedly exhausted slab") ; let key = entry . key () ; prop_assert_eq ! (used_bits ::< C > (key) , key) ; entry . insert (value) ; active . insert (key , value) ; } ActionKind :: RemoveRandom (key) => { let used_key = used_bits :: < C > (key) ; prop_assert_eq ! (slab . get (key) . map (| e | * e) , slab . get (used_key) . map (| e | * e)) ; prop_assert_eq ! (slab . remove (key) , active . remove (used_key) . is_some ()) ; } ActionKind :: RemoveExistent (seed) => { if let Some ((key , _value)) = active . remove_any (seed) { prop_assert ! (slab . contains (key)) ; prop_assert ! (slab . remove (key)) ; } } ActionKind :: TakeRandom (key) => { let used_key = used_bits :: < C > (key) ; prop_assert_eq ! (slab . get (key) . map (| e | * e) , slab . get (used_key) . map (| e | * e)) ; prop_assert_eq ! (slab . take (key) , active . remove (used_key)) ; } ActionKind :: TakeExistent (seed) => { if let Some ((key , value)) = active . remove_any (seed) { prop_assert ! (slab . contains (key)) ; prop_assert_eq ! (slab . take (key) , Some (value)) ; } } ActionKind :: GetRandom (key) => { let used_key = used_bits :: < C > (key) ; prop_assert_eq ! (slab . get (key) . map (| e | * e) , slab . get (used_key) . map (| e | * e)) ; prop_assert_eq ! (slab . get (key) . map (| e | * e) , active . get (used_key)) ; prop_assert_eq ! (slab . clone () . get_owned (key) . map (| e | * e) , active . get (used_key)) ; } ActionKind :: GetExistent (seed) => { if let Some ((key , value)) = active . get_any (seed) { prop_assert ! (slab . contains (key)) ; prop_assert_eq ! (slab . get (key) . map (| e | * e) , Some (value)) ; prop_assert_eq ! (slab . clone () . get_owned (key) . map (| e | * e) , Some (value)) ; } } } Ok (()) }
    };
}

apply_action!();