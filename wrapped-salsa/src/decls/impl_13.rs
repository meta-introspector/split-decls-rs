macro_rules! deps {
    () => {
        FxIndexSet!();
        QueryRevisionsExtra!();
        DatabaseKeyIndex!();
        CompletedQuery!();
        Durability!();
        QueryOrigin!();
        Revision!();
        ActiveQuery!();
        QueryRevisions!();
        IterationCount!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl ActiveQuery { fn new (database_key_index : DatabaseKeyIndex , iteration_count : IterationCount) -> Self { ActiveQuery { database_key_index , durability : Durability :: MAX , changed_at : Revision :: start () , input_outputs : FxIndexSet :: default () , untracked_read : false , disambiguator_map : Default :: default () , tracked_struct_ids : Default :: default () , cycle_heads : Default :: default () , iteration_count , # [cfg (feature = "accumulator")] accumulated : Default :: default () , # [cfg (feature = "accumulator")] accumulated_inputs : Default :: default () , } } fn top_into_revisions (& mut self) -> CompletedQuery { let & mut Self { database_key_index : _ , durability , changed_at , ref mut input_outputs , untracked_read , ref mut disambiguator_map , ref mut tracked_struct_ids , ref mut cycle_heads , iteration_count , # [cfg (feature = "accumulator")] ref mut accumulated , # [cfg (feature = "accumulator")] accumulated_inputs , } = self ; let origin = if untracked_read { QueryOrigin :: derived_untracked (input_outputs . drain (..) . collect ()) } else { QueryOrigin :: derived (input_outputs . drain (..) . collect ()) } ; disambiguator_map . clear () ; # [cfg (feature = "accumulator")] let accumulated_inputs = AtomicInputAccumulatedValues :: new (accumulated_inputs) ; let verified_final = cycle_heads . is_empty () ; let (active_tracked_structs , stale_tracked_structs) = tracked_struct_ids . drain () ; let extra = QueryRevisionsExtra :: new (# [cfg (feature = "accumulator")] mem :: take (accumulated) , active_tracked_structs , mem :: take (cycle_heads) , iteration_count ,) ; let revisions = QueryRevisions { changed_at , durability , origin , # [cfg (feature = "accumulator")] accumulated_inputs , verified_final : AtomicBool :: new (verified_final) , extra , } ; CompletedQuery { revisions , stale_tracked_structs , } } fn clear (& mut self) { let Self { database_key_index : _ , durability : _ , changed_at : _ , input_outputs , untracked_read : _ , disambiguator_map , tracked_struct_ids , cycle_heads , iteration_count , # [cfg (feature = "accumulator")] accumulated , # [cfg (feature = "accumulator")] accumulated_inputs : _ , } = self ; input_outputs . clear () ; disambiguator_map . clear () ; tracked_struct_ids . clear () ; * cycle_heads = Default :: default () ; * iteration_count = IterationCount :: initial () ; # [cfg (feature = "accumulator")] accumulated . clear () ; } fn reset_for (& mut self , new_database_key_index : DatabaseKeyIndex , new_iteration_count : IterationCount ,) { let Self { database_key_index , durability , changed_at , input_outputs , untracked_read , disambiguator_map , tracked_struct_ids , cycle_heads , iteration_count , # [cfg (feature = "accumulator")] accumulated , # [cfg (feature = "accumulator")] accumulated_inputs , } = self ; * database_key_index = new_database_key_index ; * durability = Durability :: MAX ; * changed_at = Revision :: start () ; * untracked_read = false ; * iteration_count = new_iteration_count ; debug_assert ! (input_outputs . is_empty () , "`ActiveQuery::clear` or `ActiveQuery::into_revisions` should've been called") ; debug_assert ! (disambiguator_map . is_empty () , "`ActiveQuery::clear` or `ActiveQuery::into_revisions` should've been called") ; debug_assert ! (tracked_struct_ids . is_empty () , "`ActiveQuery::clear` or `ActiveQuery::into_revisions` should've been called") ; debug_assert ! (cycle_heads . is_empty () , "`ActiveQuery::clear` or `ActiveQuery::into_revisions` should've been called") ; # [cfg (feature = "accumulator")] { * accumulated_inputs = Default :: default () ; debug_assert ! (accumulated . is_empty () , "`ActiveQuery::clear` or `ActiveQuery::into_revisions` should've been called") ; } } }
    };
}

impl_13!();