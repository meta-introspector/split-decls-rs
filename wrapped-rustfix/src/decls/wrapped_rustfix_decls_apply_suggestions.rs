use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Applies multiple `suggestions` to the given `code`, handling certain conflicts automatically.
///
/// If a replacement in a suggestion exactly matches a replacement of a previously applied solution,
/// that entire suggestion will be skipped without generating an error.
/// This is currently done to alleviate issues like rust-lang/rust#51211,
/// although it may be removed if that's fixed deeper in the compiler.
///
/// The intent of this design is that the overall application process
/// should repeatedly apply non-conflicting suggestions then rëevaluate the result,
/// looping until either there are no more suggestions to apply or some budget is exhausted.
pub fn apply_suggestions(code: &str, suggestions: &[Suggestion]) -> Result<String, Error> {
    let mut fix = CodeFix::new(code);
    for suggestion in suggestions.iter().rev() {
        fix.apply(suggestion).or_else(|err| match err {
            Error::AlreadyReplaced {
                is_identical: true, ..
            } => Ok(()),
            _ => Err(err),
        })?;
    }
    fix.finish()
}
