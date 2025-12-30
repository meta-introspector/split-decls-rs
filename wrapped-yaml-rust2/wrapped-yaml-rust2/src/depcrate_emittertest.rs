// Generated macro for test (module)
macro_rules! Depcrate_emittertest {
() => {
// Module: crate::emitter
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: YamlEmitter ; use crate :: YamlLoader ; # [test] fn test_multiline_string () { let input = r#"{foo: "bar!\nbar!", baz: 42}"# ; let parsed = YamlLoader :: load_from_str (input) . unwrap () ; let mut output = String :: new () ; let mut emitter = YamlEmitter :: new (& mut output) ; emitter . multiline_strings (true) ; emitter . dump (& parsed [0]) . unwrap () ; } }
};
}
