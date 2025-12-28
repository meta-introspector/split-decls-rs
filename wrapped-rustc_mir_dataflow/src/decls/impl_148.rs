macro_rules! deps {
    () => {
        MaybeUninitializedPlaces!();
        DropFlagState!();
        Analysis!();
        InactiveVariants!();
        MaybePlacesSwitchIntData!();
        MaybeUninitializedPlacesDomain!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < 'tcx > Analysis < 'tcx > for MaybeUninitializedPlaces < '_ , 'tcx > { type Domain = MaybeUninitializedPlacesDomain ; type SwitchIntData = MaybePlacesSwitchIntData < 'tcx > ; const NAME : & 'static str = "maybe_uninit" ; fn bottom_value (& self , _ : & mir :: Body < 'tcx >) -> Self :: Domain { MixedBitSet :: new_empty (self . move_data () . move_paths . len ()) } fn initialize_start_block (& self , _ : & mir :: Body < 'tcx > , state : & mut Self :: Domain) { state . insert_all () ; drop_flag_effects_for_function_entry (self . body , self . move_data , | path , s | { assert ! (s == DropFlagState :: Present) ; state . remove (path) ; }) ; } fn apply_primary_statement_effect (& mut self , state : & mut Self :: Domain , _statement : & mir :: Statement < 'tcx > , location : Location ,) { drop_flag_effects_for_location (self . body , self . move_data , location , | path , s | { Self :: update_bits (state , path , s) }) ; } fn apply_primary_terminator_effect < 'mir > (& mut self , state : & mut Self :: Domain , terminator : & 'mir mir :: Terminator < 'tcx > , location : Location ,) -> TerminatorEdges < 'mir , 'tcx > { drop_flag_effects_for_location (self . body , self . move_data , location , | path , s | { Self :: update_bits (state , path , s) }) ; if self . skip_unreachable_unwind . contains (location . block) { let mir :: TerminatorKind :: Drop { target , unwind , .. } = terminator . kind else { bug ! () } ; assert_matches ! (unwind , mir :: UnwindAction :: Cleanup (_)) ; TerminatorEdges :: Single (target) } else { terminator . edges () } } fn apply_call_return_effect (& mut self , state : & mut Self :: Domain , _block : mir :: BasicBlock , return_places : CallReturnPlaces < '_ , 'tcx > ,) { return_places . for_each (| place | { on_lookup_result_bits (self . move_data () , self . move_data () . rev_lookup . find (place . as_ref ()) , | mpi | { state . kill (mpi) ; } ,) ; }) ; } fn get_switch_int_data (& mut self , block : mir :: BasicBlock , discr : & mir :: Operand < 'tcx > ,) -> Option < Self :: SwitchIntData > { if ! self . tcx . sess . opts . unstable_opts . precise_enum_drop_elaboration { return None ; } if ! self . mark_inactive_variants_as_uninit { return None ; } MaybePlacesSwitchIntData :: new (self . tcx , self . body , block , discr) } fn apply_switch_int_edge_effect (& mut self , data : & mut Self :: SwitchIntData , state : & mut Self :: Domain , value : SwitchTargetValue , targets : & mir :: SwitchTargets ,) { let inactive_variants = match value { SwitchTargetValue :: Normal (value) => InactiveVariants :: Active (data . next_discr (value)) , SwitchTargetValue :: Otherwise if self . include_inactive_in_otherwise => { InactiveVariants :: Inactives (data . variants (targets)) } _ => return , } ; drop_flag_effects :: on_all_inactive_variants (self . move_data , data . enum_place , & inactive_variants , | mpi | state . gen_ (mpi) ,) ; } }
    };
}

impl_148!()