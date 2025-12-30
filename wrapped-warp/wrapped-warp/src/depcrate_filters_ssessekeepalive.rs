// Generated macro for SseKeepAlive (struct)
macro_rules! Depcrate_filters_sseSseKeepAlive {
() => {
// Module: crate::filters::sse
// Provides: {"SseKeepAlive"}
// Dependencies: {}
# [allow (missing_debug_implementations)] # [pin_project] struct SseKeepAlive < S > { # [pin] event_stream : S , comment_text : Cow < 'static , str > , max_interval : Duration , # [pin] alive_timer : Sleep , }
};
}
