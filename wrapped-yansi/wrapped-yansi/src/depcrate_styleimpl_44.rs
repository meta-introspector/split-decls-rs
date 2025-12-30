// Generated macro for impl_44 (impl)
macro_rules! Depcrate_styleimpl_44 {
() => {
// Module: crate::style
// Provides: {"impl_44"}
// Dependencies: {}
impl PartialOrd for Style { fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { let Style { foreground : fg_a , background : bg_a , attributes : attrs_a , quirks : _ , condition : _ , } = self ; let Style { foreground : fg_b , background : bg_b , attributes : attrs_b , quirks : _ , condition : _ , } = other ; match fg_a . partial_cmp (& fg_b) { Some (core :: cmp :: Ordering :: Equal) => { } ord => return ord , } match bg_a . partial_cmp (& bg_b) { Some (core :: cmp :: Ordering :: Equal) => { } ord => return ord , } attrs_a . partial_cmp (& attrs_b) } }
};
}
