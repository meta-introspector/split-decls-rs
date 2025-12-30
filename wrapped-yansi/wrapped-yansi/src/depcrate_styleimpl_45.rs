// Generated macro for impl_45 (impl)
macro_rules! Depcrate_styleimpl_45 {
() => {
// Module: crate::style
// Provides: {"impl_45"}
// Dependencies: {}
impl Ord for Style { fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { let Style { foreground : fg_a , background : bg_a , attributes : attrs_a , quirks : _ , condition : _ , } = self ; let Style { foreground : fg_b , background : bg_b , attributes : attrs_b , quirks : _ , condition : _ , } = other ; match fg_a . cmp (& fg_b) { core :: cmp :: Ordering :: Equal => { } ord => return ord , } match bg_a . cmp (& bg_b) { core :: cmp :: Ordering :: Equal => { } ord => return ord , } attrs_a . cmp (& attrs_b) } }
};
}
