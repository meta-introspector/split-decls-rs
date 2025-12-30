// Generated macro for check_lowercase (function)
macro_rules! Depcrate_fluent_lowercasecheck_lowercase {
() => {
// Module: crate::fluent_lowercase
// Provides: {"check_lowercase"}
// Dependencies: {}
fn check_lowercase (filename : & str , contents : & str , bad : & mut bool) { let (Ok (parse) | Err ((parse , _))) = fluent_syntax :: parser :: parse (contents) ; for entry in & parse . body { if let Entry :: Message (msg) = entry && let Message { value : Some (pattern) , .. } = msg && let [first_pattern , ..] = & pattern . elements [..] && let PatternElement :: TextElement { value } = first_pattern && value . chars () . next () . is_some_and (char :: is_uppercase) && ! is_allowed_capitalized_word (value) { tidy_error ! (bad , "{filename}: message `{value}` starts with an uppercase letter. Fix it or add it to `ALLOWED_CAPITALIZED_WORDS`") ; } } }
};
}
