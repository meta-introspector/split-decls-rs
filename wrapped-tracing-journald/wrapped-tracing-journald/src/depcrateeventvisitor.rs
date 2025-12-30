// Generated macro for EventVisitor (struct)
macro_rules! DepcrateEventVisitor {
() => {
// Module: crate
// Provides: {"EventVisitor"}
// Dependencies: {}
# [doc = " Helper for generating the journal export format, which is consumed by journald:"] # [doc = " https://www.freedesktop.org/wiki/Software/systemd/export/"] struct EventVisitor < 'a > { buf : & 'a mut Vec < u8 > , prefix : Option < & 'a str > , }
};
}
