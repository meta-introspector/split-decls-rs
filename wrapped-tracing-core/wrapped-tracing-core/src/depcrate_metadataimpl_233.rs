// Generated macro for impl_233 (impl)
macro_rules! Depcrate_metadataimpl_233 {
() => {
// Module: crate::metadata
// Provides: {"impl_233"}
// Dependencies: {}
impl Kind { const EVENT_BIT : u8 = 1 << 0 ; const SPAN_BIT : u8 = 1 << 1 ; const HINT_BIT : u8 = 1 << 2 ; # [doc = " `Event` callsite"] pub const EVENT : Kind = Kind (Self :: EVENT_BIT) ; # [doc = " `Span` callsite"] pub const SPAN : Kind = Kind (Self :: SPAN_BIT) ; # [doc = " `enabled!` callsite. [`Subscriber`][`crate::subscriber::Subscriber`]s can assume"] # [doc = " this `Kind` means they will never receive a"] # [doc = " full event with this [`Metadata`]."] pub const HINT : Kind = Kind (Self :: HINT_BIT) ; # [doc = " Return true if the callsite kind is `Span`"] pub fn is_span (& self) -> bool { self . 0 & Self :: SPAN_BIT == Self :: SPAN_BIT } # [doc = " Return true if the callsite kind is `Event`"] pub fn is_event (& self) -> bool { self . 0 & Self :: EVENT_BIT == Self :: EVENT_BIT } # [doc = " Return true if the callsite kind is `Hint`"] pub fn is_hint (& self) -> bool { self . 0 & Self :: HINT_BIT == Self :: HINT_BIT } # [doc = " Sets that this `Kind` is a [hint](Self::HINT)."] # [doc = ""] # [doc = " This can be called on [`SPAN`](Self::SPAN) and [`EVENT`](Self::EVENT)"] # [doc = " kinds to construct a hint callsite that also counts as a span or event."] pub const fn hint (self) -> Self { Self (self . 0 | Self :: HINT_BIT) } }
};
}
