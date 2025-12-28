macro_rules! deps {
    () => {
        AvailableDepth!();
        WithOverflow!();
        NestedGoals!();
        Success!();
        CacheData!();
        EvaluationResult!();
        GlobalCache!();
        Cx!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < X : Cx > GlobalCache < X > { # [doc = " Insert a final result into the global cache."] pub (super) fn insert (& mut self , cx : X , input : X :: Input , evaluation_result : EvaluationResult < X > , dep_node : X :: DepNodeIndex ,) { let EvaluationResult { encountered_overflow , required_depth , heads , nested_goals , result } = evaluation_result ; debug_assert ! (heads . is_empty ()) ; let result = cx . mk_tracked (result , dep_node) ; let entry = self . map . entry (input) . or_default () ; if encountered_overflow { let with_overflow = WithOverflow { nested_goals , result } ; let prev = entry . with_overflow . insert (required_depth , with_overflow) ; if let Some (prev) = & prev { assert ! (cx . evaluation_is_concurrent ()) ; assert_eq ! (cx . get_tracked (& prev . result) , evaluation_result . result) ; } } else { let prev = entry . success . replace (Success { required_depth , nested_goals , result }) ; if let Some (prev) = & prev { assert ! (cx . evaluation_is_concurrent ()) ; assert_eq ! (cx . get_tracked (& prev . result) , evaluation_result . result) ; } } } # [doc = " Try to fetch a cached result, checking the recursion limit"] # [doc = " and handling root goals of coinductive cycles."] # [doc = ""] # [doc = " If this returns `Some` the cache result can be used."] pub (super) fn get < 'a > (& 'a self , cx : X , input : X :: Input , available_depth : AvailableDepth , mut candidate_is_applicable : impl FnMut (& NestedGoals < X >) -> bool ,) -> Option < CacheData < 'a , X > > { let entry = self . map . get (& input) ? ; if let Some (Success { required_depth , ref nested_goals , ref result }) = entry . success && available_depth . cache_entry_is_applicable (required_depth) && candidate_is_applicable (nested_goals) { return Some (CacheData { result : cx . get_tracked (& result) , required_depth , encountered_overflow : false , nested_goals , }) ; } let additional_depth = available_depth . 0 ; if let Some (WithOverflow { nested_goals , result }) = entry . with_overflow . get (& additional_depth) && candidate_is_applicable (nested_goals) { return Some (CacheData { result : cx . get_tracked (result) , required_depth : additional_depth , encountered_overflow : true , nested_goals , }) ; } None } }
    };
}

impl_139!();