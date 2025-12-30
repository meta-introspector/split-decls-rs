// Generated macro for new_creator (function)
macro_rules! Depcrate_test_utilsnew_creator {
() => {
// Module: crate::test::utils
// Provides: {"new_creator"}
// Dependencies: {}
pub fn new_creator () -> Arc < Mutex < MockCommandCreator > > { let client = Client :: new () ; Arc :: new (Mutex :: new (MockCommandCreator :: new (& client))) }
};
}
