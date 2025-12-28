macro_rules! deps {
    () => {
        GlobalCache!();
        Interner!();
    };
}

macro_rules! Cx {
    () => {
        deps!();
        # [doc = " The search graph does not simply use `Interner` directly"] # [doc = " to enable its fuzzing without having to stub the rest of"] # [doc = " the interner. We don't make this a super trait of `Interner`"] # [doc = " as users of the shared type library shouldn't have to care"] # [doc = " about `Input` and `Result` as they are implementation details"] # [doc = " of the search graph."] pub trait Cx : Copy { type Input : Debug + Eq + Hash + Copy ; type Result : Debug + Eq + Hash + Copy ; type DepNodeIndex ; type Tracked < T : Debug + Clone > : Debug ; fn mk_tracked < T : Debug + Clone > (self , data : T , dep_node_index : Self :: DepNodeIndex ,) -> Self :: Tracked < T > ; fn get_tracked < T : Debug + Clone > (self , tracked : & Self :: Tracked < T >) -> T ; fn with_cached_task < T > (self , task : impl FnOnce () -> T) -> (T , Self :: DepNodeIndex) ; fn with_global_cache < R > (self , f : impl FnOnce (& mut GlobalCache < Self >) -> R) -> R ; fn evaluation_is_concurrent (& self) -> bool ; }
    };
}

Cx!();