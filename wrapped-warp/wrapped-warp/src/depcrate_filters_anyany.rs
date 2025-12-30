// Generated macro for any (function)
macro_rules! Depcrate_filters_anyany {
() => {
// Module: crate::filters::any
// Provides: {"any"}
// Dependencies: {}
# [doc = " A [`Filter`] that matches any route."] # [doc = ""] # [doc = " This can be a useful building block to build new filters from,"] # [doc = " since [`Filter`] is otherwise a sealed trait."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let route = warp::any()"] # [doc = "     .map(|| {"] # [doc = "         \"I always return this string!\""] # [doc = "     });"] # [doc = " ```"] # [doc = ""] # [doc = " This could allow creating a single `impl Filter` returning a specific"] # [doc = " reply, that can then be used as the end of several different filter"] # [doc = " chains."] # [doc = ""] # [doc = " Another use case is turning some clone-able resource into a `Filter`,"] # [doc = " thus allowing to easily `and` it together with others."] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Arc;"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let state = Arc::new(vec![33, 41]);"] # [doc = " let with_state = warp::any().map(move || state.clone());"] # [doc = ""] # [doc = " // Now we could `and` with any other filter:"] # [doc = ""] # [doc = " let route = warp::path::param()"] # [doc = "     .and(with_state)"] # [doc = "     .map(|param_id: u32, db: Arc<Vec<u32>>| {"] # [doc = "         db.contains(&param_id)"] # [doc = "     });"] # [doc = " ```"] pub fn any () -> impl Filter < Extract = () , Error = Infallible > + Copy { Any }
};
}
