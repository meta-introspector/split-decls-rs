macro_rules! deps {
    () => {
        MovePathIndexAtBlock!();
        DropsReachable!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < 'a , 'mir , 'tcx > DropsReachable < 'a , 'mir , 'tcx > { fn visit (& mut self , block : BasicBlock) { let move_set_size = self . move_data . move_paths . len () ; let make_new_path_set = | | Rc :: new (RefCell :: new (MixedBitSet :: new_empty (move_set_size))) ; let data = & self . body . basic_blocks [block] ; let Some (terminator) = & data . terminator else { return } ; let dropped_local_here = Rc :: clone (self . visited . entry (block) . or_insert_with (make_new_path_set)) ; match self . block_drop_value_info [block] { MovePathIndexAtBlock :: Some (dropped) => { dropped_local_here . borrow_mut () . insert (dropped) ; } MovePathIndexAtBlock :: Unknown => { if let TerminatorKind :: Drop { place , .. } = & terminator . kind && let LookupResult :: Exact (idx) | LookupResult :: Parent (Some (idx)) = self . move_data . rev_lookup . find (place . as_ref ()) { self . maybe_init . seek_before_primary_effect (Location { block , statement_index : data . statements . len () , }) ; if let MaybeReachable :: Reachable (maybe_init) = self . maybe_init . get () && maybe_init . contains (idx) { self . block_drop_value_info [block] = MovePathIndexAtBlock :: Some (idx) ; dropped_local_here . borrow_mut () . insert (idx) ; } else { self . block_drop_value_info [block] = MovePathIndexAtBlock :: None ; } } } MovePathIndexAtBlock :: None => { } } for succ in terminator . successors () { let target = & self . body . basic_blocks [succ] ; if target . is_cleanup { continue ; } let dropped_local_there = match self . visited . entry (succ) { hash_map :: Entry :: Occupied (occupied_entry) => { if succ == block || ! occupied_entry . get () . borrow_mut () . union (& * dropped_local_here . borrow ()) { continue ; } Rc :: clone (occupied_entry . get ()) } hash_map :: Entry :: Vacant (vacant_entry) => Rc :: clone (vacant_entry . insert (Rc :: new (RefCell :: new (dropped_local_here . borrow () . clone ()))) ,) , } ; if let Some (terminator) = & target . terminator && let TerminatorKind :: Drop { place : dropped_place , target : _ , unwind : _ , replace : _ , drop : _ , async_fut : _ , } = & terminator . kind && place_has_common_prefix (dropped_place , self . place) { self . collected_drops . union (& * dropped_local_there . borrow ()) ; if self . drop_span . is_none () { * self . drop_span = Some (terminator . source_info . span) ; } } else { self . visit (succ) } } } }
    };
}

impl_86!();