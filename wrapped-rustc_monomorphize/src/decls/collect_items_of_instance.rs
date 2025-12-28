macro_rules! deps {
    () => {
        MirUsedCollector!();
        MonoItems!();
    };
}

macro_rules! collect_items_of_instance {
    () => {
        deps!();
        # [doc = " Scans the MIR in order to find function calls, closures, and drop-glue."] # [doc = ""] # [doc = " Anything that's found is added to `output`. Furthermore the \"mentioned items\" of the MIR are returned."] # [instrument (skip (tcx) , level = "debug")] fn collect_items_of_instance < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , mode : CollectionMode ,) -> (MonoItems < 'tcx > , MonoItems < 'tcx >) { tcx . ensure_ok () . check_mono_item (instance) ; let body = tcx . instance_mir (instance . def) ; let mut used_items = MonoItems :: new () ; let mut mentioned_items = MonoItems :: new () ; let mut used_mentioned_items = Default :: default () ; let mut collector = MirUsedCollector { tcx , body , used_items : & mut used_items , used_mentioned_items : & mut used_mentioned_items , instance , } ; if mode == CollectionMode :: UsedItems { if tcx . sess . opts . debuginfo == DebugInfo :: Full { for var_debug_info in & body . var_debug_info { collector . visit_var_debug_info (var_debug_info) ; } } for (bb , data) in traversal :: mono_reachable (body , tcx , instance) { collector . visit_basic_block_data (bb , data) } } for const_op in body . required_consts () { if let Some (val) = collector . eval_constant (const_op) { collect_const_value (tcx , val , & mut mentioned_items) ; } } for item in body . mentioned_items () { if ! collector . used_mentioned_items . contains (& item . node) { let item_mono = collector . monomorphize (item . node) ; visit_mentioned_item (tcx , & item_mono , item . span , & mut mentioned_items) ; } } (used_items , mentioned_items) }
    };
}

collect_items_of_instance!();