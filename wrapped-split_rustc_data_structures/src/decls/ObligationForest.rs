macro_rules! deps {
    () => {
        ObligationTreeId!();
        ForestObligation!();
        Node!();
    };
}

macro_rules! ObligationForest {
    () => {
        deps!();
        pub struct ObligationForest < O : ForestObligation > { # [doc = " The list of obligations. In between calls to [Self::process_obligations],"] # [doc = " this list only contains nodes in the `Pending` or `Waiting` state."] # [doc = ""] # [doc = " `usize` indices are used here and throughout this module, rather than"] # [doc = " [`rustc_index::newtype_index!`] indices, because this code is hot enough"] # [doc = " that the `u32`-to-`usize` conversions that would be required are"] # [doc = " significant, and space considerations are not important."] nodes : Vec < Node < O > > , # [doc = " A cache of predicates that have been successfully completed."] done_cache : FxHashSet < O :: CacheKey > , # [doc = " A cache of the nodes in `nodes`, indexed by predicate. Unfortunately,"] # [doc = " its contents are not guaranteed to match those of `nodes`. See the"] # [doc = " comments in `Self::process_obligation` for details."] active_cache : FxHashMap < O :: CacheKey , usize > , # [doc = " A vector reused in [Self::compress()] and [Self::find_cycles_from_node()],"] # [doc = " to avoid allocating new vectors."] reused_node_vec : Vec < usize > , obligation_tree_id_generator : ObligationTreeIdGenerator , # [doc = " Per tree error cache. This is used to deduplicate errors,"] # [doc = " which is necessary to avoid trait resolution overflow in"] # [doc = " some cases."] # [doc = ""] # [doc = " See [this][details] for details."] # [doc = ""] # [doc = " [details]: https://github.com/rust-lang/rust/pull/53255#issuecomment-421184780"] error_cache : FxHashMap < ObligationTreeId , FxHashSet < O :: CacheKey > > , }
    };
}

ObligationForest!();