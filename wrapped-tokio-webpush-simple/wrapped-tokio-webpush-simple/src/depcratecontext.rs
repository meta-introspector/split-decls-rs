// Generated macro for Context (struct)
macro_rules! DepcrateContext {
() => {
// Module: crate
// Provides: {"Context"}
// Dependencies: {}
struct Context { channels : RefCell < HashMap < Uuid , Channel > > , uaids : RefCell < HashMap < Uuid , Client > > , }
};
}
