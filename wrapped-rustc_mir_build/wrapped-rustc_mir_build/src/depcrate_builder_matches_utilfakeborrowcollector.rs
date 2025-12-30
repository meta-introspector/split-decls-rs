// Generated macro for FakeBorrowCollector (struct)
macro_rules! Depcrate_builder_matches_utilFakeBorrowCollector {
() => {
// Module: crate::builder::matches::util
// Provides: {"FakeBorrowCollector"}
// Dependencies: {}
pub (super) struct FakeBorrowCollector < 'a , 'b , 'tcx > { cx : & 'a mut Builder < 'b , 'tcx > , # [doc = " Base of the scrutinee place. Used to distinguish bindings inside the scrutinee place from"] # [doc = " bindings inside deref patterns."] scrutinee_base : PlaceBase , # [doc = " Store for each place the kind of borrow to take. In case of conflicts, we take the strongest"] # [doc = " borrow (i.e. Deep > Shallow)."] # [doc = " Invariant: for any place in `fake_borrows`, all the prefixes of this place that are"] # [doc = " dereferences are also borrowed with the same of stronger borrow kind."] fake_borrows : FxIndexMap < Place < 'tcx > , FakeBorrowKind > , }
};
}
