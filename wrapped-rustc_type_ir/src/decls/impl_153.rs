macro_rules! deps {
    () => {
        CycleHead!();
        CycleHeads!();
        HeadUsages!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl CycleHeads { fn is_empty (& self) -> bool { self . heads . is_empty () } fn highest_cycle_head (& self) -> (StackDepth , CycleHead) { self . heads . last_key_value () . map (| (k , v) | (* k , * v)) . unwrap () } fn highest_cycle_head_index (& self) -> StackDepth { self . opt_highest_cycle_head_index () . unwrap () } fn opt_highest_cycle_head_index (& self) -> Option < StackDepth > { self . heads . last_key_value () . map (| (k , _) | * k) } fn opt_lowest_cycle_head_index (& self) -> Option < StackDepth > { self . heads . first_key_value () . map (| (k , _) | * k) } fn remove_highest_cycle_head (& mut self) -> CycleHead { let last = self . heads . pop_last () ; last . unwrap () . 1 } fn insert (& mut self , head_index : StackDepth , path_from_entry : impl Into < PathsToNested > + Copy , usages : HeadUsages ,) { match self . heads . entry (head_index) { btree_map :: Entry :: Vacant (entry) => { entry . insert (CycleHead { paths_to_head : path_from_entry . into () , usages }) ; } btree_map :: Entry :: Occupied (entry) => { let head = entry . into_mut () ; head . paths_to_head |= path_from_entry . into () ; head . usages . add_usages_from_nested (usages) ; } } } fn ignore_usages (& mut self , head_index : StackDepth , usages : HeadUsages) { self . heads . get_mut (& head_index) . unwrap () . usages . ignore_usages (usages) } fn iter (& self) -> impl Iterator < Item = (StackDepth , CycleHead) > + '_ { self . heads . iter () . map (| (k , v) | (* k , * v)) } }
    };
}

impl_153!()