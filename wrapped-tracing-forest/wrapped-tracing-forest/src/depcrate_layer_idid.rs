// Generated macro for id (function)
macro_rules! Depcrate_layer_idid {
() => {
// Module: crate::layer::id
// Provides: {"id"}
// Dependencies: {}
# [doc = " Gets the current [`Uuid`] of an entered span within a `tracing-forest`"] # [doc = " subscriber."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Passing in a `Uuid` to a span, and then retreiving it from within the span:"] # [doc = " ```"] # [doc = " # use tracing::{info, info_span};"] # [doc = " # use uuid::Uuid;"] # [doc = " # tracing_forest::init();"] # [doc = " let uuid = Uuid::new_v4();"] # [doc = ""] # [doc = " // Tracing's syntax allows us to omit the redundent naming of the field here"] # [doc = " info_span!(\"my_span\", %uuid).in_scope(|| {"] # [doc = "     assert!(tracing_forest::id() == uuid);"] # [doc = " });"] # [doc = " ```"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function panics if there is no current subscriber, if the subscriber"] # [doc = " isn't composed with a [`ForestLayer`], or if the subscriber isn't in a span."] # [doc = ""] # [doc = " [`ForestLayer`]: crate::layer::ForestLayer"] # [must_use] pub fn id () -> Uuid { tracing :: dispatcher :: get_default (| dispatch | { let subscriber = dispatch . downcast_ref :: < Registry > () . unwrap_or_else (fail :: subscriber_not_found) ; let current = subscriber . current_span () ; let id = current . id () . expect (fail :: NO_CURRENT_SPAN) ; subscriber . span (id) . expect (fail :: SPAN_NOT_IN_CONTEXT) . extensions () . get :: < OpenedSpan > () . expect (fail :: NO_FOREST_LAYER) . uuid () }) }
};
}
