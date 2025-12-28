macro_rules! deps {
    () => {
        EverInitializedPlaces!();
        Analysis!();
        EverInitializedPlacesDomain!();
        InitKind!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < 'tcx > Analysis < 'tcx > for EverInitializedPlaces < '_ , 'tcx > { type Domain = EverInitializedPlacesDomain ; const NAME : & 'static str = "ever_init" ; fn bottom_value (& self , _ : & mir :: Body < 'tcx >) -> Self :: Domain { MixedBitSet :: new_empty (self . move_data () . inits . len ()) } fn initialize_start_block (& self , body : & mir :: Body < 'tcx > , state : & mut Self :: Domain) { for arg_init in 0 .. body . arg_count { state . insert (InitIndex :: new (arg_init)) ; } } # [instrument (skip (self , state) , level = "debug")] fn apply_primary_statement_effect (& mut self , state : & mut Self :: Domain , stmt : & mir :: Statement < 'tcx > , location : Location ,) { let move_data = self . move_data () ; let init_path_map = & move_data . init_path_map ; let init_loc_map = & move_data . init_loc_map ; let rev_lookup = & move_data . rev_lookup ; debug ! ("initializes move_indexes {:?}" , init_loc_map [location]) ; state . gen_all (init_loc_map [location] . iter () . copied ()) ; if let mir :: StatementKind :: StorageDead (local) = stmt . kind && let Some (move_path_index) = rev_lookup . find_local (local) { debug ! ("clears the ever initialized status of {:?}" , init_path_map [move_path_index]) ; state . kill_all (init_path_map [move_path_index] . iter () . copied ()) ; } } # [instrument (skip (self , state , terminator) , level = "debug")] fn apply_primary_terminator_effect < 'mir > (& mut self , state : & mut Self :: Domain , terminator : & 'mir mir :: Terminator < 'tcx > , location : Location ,) -> TerminatorEdges < 'mir , 'tcx > { let (body , move_data) = (self . body , self . move_data ()) ; let term = body [location . block] . terminator () ; let init_loc_map = & move_data . init_loc_map ; debug ! (? term) ; debug ! ("initializes move_indexes {:?}" , init_loc_map [location]) ; state . gen_all (init_loc_map [location] . iter () . filter (| init_index | { move_data . inits [* * init_index] . kind != InitKind :: NonPanicPathOnly }) . copied () ,) ; terminator . edges () } fn apply_call_return_effect (& mut self , state : & mut Self :: Domain , block : mir :: BasicBlock , _return_places : CallReturnPlaces < '_ , 'tcx > ,) { let move_data = self . move_data () ; let init_loc_map = & move_data . init_loc_map ; let call_loc = self . body . terminator_loc (block) ; for init_index in & init_loc_map [call_loc] { state . gen_ (* init_index) ; } } }
    };
}

impl_150!()