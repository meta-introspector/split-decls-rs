// Generated macro for ListenerFn (type)
macro_rules! DepcrateListenerFn {
() => {
// Module: crate
// Provides: {"ListenerFn"}
// Dependencies: {}
# [doc = " A callback function that is called when a rule is matched."] # [doc = " The first argument is the name of the rule and the second is the span of the rule."] # [doc = " The function should return `true` if parsing should be terminated"] # [doc = " (if the new parsing session was started) or `false` otherwise."] type ListenerFn = Box < dyn Fn (String , & Position < '_ >) -> bool + Sync + Send + RefUnwindSafe + UnwindSafe > ;
};
}
