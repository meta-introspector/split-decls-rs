// Generated macro for format_coro (function)
macro_rules! Depcrate_utilsformat_coro {
() => {
// Module: crate::utils
// Provides: {"format_coro"}
// Dependencies: {}
# [inline] pub (crate) fn format_coro (coroutine_kind : & ast :: CoroutineKind) -> & 'static str { match coroutine_kind { ast :: CoroutineKind :: Async { .. } => "async " , ast :: CoroutineKind :: Gen { .. } => "gen " , ast :: CoroutineKind :: AsyncGen { .. } => "async gen " , } }
};
}
