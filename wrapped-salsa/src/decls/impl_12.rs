macro_rules! deps {
    () => {
        Stamp!();
        CycleHeads!();
        IdentityHash!();
        DatabaseKeyIndex!();
        Durability!();
        IdentityMap!();
        Accumulator!();
        QueryEdge!();
        Identity!();
        Id!();
        IngredientIndex!();
        ActiveQuery!();
        Revision!();
        Disambiguator!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl ActiveQuery { pub (super) fn seed_iteration (& mut self , durability : Durability , changed_at : Revision , edges : & [QueryEdge] , untracked_read : bool , active_tracked_ids : & [(Identity , Id)] ,) { assert ! (self . input_outputs . is_empty ()) ; self . input_outputs . extend (edges . iter () . cloned ()) ; self . durability = self . durability . min (durability) ; self . changed_at = self . changed_at . max (changed_at) ; self . untracked_read |= untracked_read ; self . tracked_struct_ids . mark_all_active (active_tracked_ids . iter () . copied ()) ; } pub (super) fn take_cycle_heads (& mut self) -> CycleHeads { std :: mem :: take (& mut self . cycle_heads) } pub (super) fn add_read (& mut self , input : DatabaseKeyIndex , durability : Durability , changed_at : Revision , cycle_heads : & CycleHeads , # [cfg (feature = "accumulator")] has_accumulated : bool , # [cfg (feature = "accumulator")] accumulated_inputs : & AtomicInputAccumulatedValues ,) { self . durability = self . durability . min (durability) ; self . changed_at = self . changed_at . max (changed_at) ; self . input_outputs . insert (QueryEdge :: input (input)) ; self . cycle_heads . extend (cycle_heads) ; # [cfg (feature = "accumulator")] { self . accumulated_inputs = self . accumulated_inputs . or_else (| | match has_accumulated { true => InputAccumulatedValues :: Any , false => accumulated_inputs . load () , }) ; } } pub (super) fn add_read_simple (& mut self , input : DatabaseKeyIndex , durability : Durability , revision : Revision ,) { self . durability = self . durability . min (durability) ; self . changed_at = self . changed_at . max (revision) ; self . input_outputs . insert (QueryEdge :: input (input)) ; } pub (super) fn add_untracked_read (& mut self , changed_at : Revision) { self . untracked_read = true ; self . durability = Durability :: MIN ; self . changed_at = changed_at ; } pub (super) fn add_synthetic_read (& mut self , durability : Durability , revision : Revision) { self . untracked_read = true ; self . durability = self . durability . min (durability) ; self . changed_at = self . changed_at . max (revision) ; } # [cfg (feature = "accumulator")] pub (super) fn accumulate (& mut self , index : crate :: IngredientIndex , value : impl Accumulator) { self . accumulated . accumulate (index , value) ; } # [doc = " Adds a key to our list of outputs."] pub (super) fn add_output (& mut self , key : DatabaseKeyIndex) { self . input_outputs . insert (QueryEdge :: output (key)) ; } # [doc = " True if the given key was output by this query."] pub (super) fn disambiguate (& mut self , key : IdentityHash) -> Disambiguator { self . disambiguator_map . disambiguate (key) } pub (super) fn stamp (& self) -> Stamp { Stamp { durability : self . durability , changed_at : self . changed_at , } } pub (crate) fn tracked_struct_ids (& self) -> & IdentityMap { & self . tracked_struct_ids } pub (crate) fn tracked_struct_ids_mut (& mut self) -> & mut IdentityMap { & mut self . tracked_struct_ids } }
    };
}

impl_12!();