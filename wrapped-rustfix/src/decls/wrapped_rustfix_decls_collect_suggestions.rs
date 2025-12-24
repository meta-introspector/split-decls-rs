use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Collects code [`Suggestion`]s from a single compiler diagnostic line.
///
/// * `only` --- only diagnostics with code in a set of error codes would be collected.
pub fn collect_suggestions<S: ::std::hash::BuildHasher>(
    diagnostic: &Diagnostic,
    only: &HashSet<String, S>,
    filter: Filter,
) -> Option<Suggestion> {
    if !only.is_empty() {
        if let Some(ref code) = diagnostic.code {
            if !only.contains(&code.code) {
                return None;
            }
        } else {
            return None;
        }
    }
    let solutions: Vec<_> = diagnostic
        .children
        .iter()
        .filter_map(|child| {
            let replacements: Vec<_> = child
                .spans
                .iter()
                .filter(|span| {
                    use crate::Filter::*;
                    use crate::diagnostics::Applicability::*;
                    match (filter, &span.suggestion_applicability) {
                        (MachineApplicableOnly, Some(MachineApplicable)) => true,
                        (MachineApplicableOnly, _) => false,
                        (Everything, _) => true,
                    }
                })
                .filter_map(collect_span)
                .collect();
            if !replacements.is_empty() {
                Some(Solution {
                    message: child.message.clone(),
                    replacements,
                })
            } else {
                None
            }
        })
        .collect();
    if solutions.is_empty() {
        None
    } else {
        Some(Suggestion {
            message: diagnostic.message.clone(),
            snippets: diagnostic.spans.iter().map(span_to_snippet).collect(),
            solutions,
        })
    }
}
