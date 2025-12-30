// Generated macro for KeySchedule (struct)
macro_rules! Depcrate_tls13_key_scheduleKeySchedule {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"KeySchedule"}
// Dependencies: {}
# [doc = " This is the TLS1.3 key schedule.  It stores the current secret and"] # [doc = " the type of hash.  This isn't used directly; but only through the"] # [doc = " typestates."] struct KeySchedule { current : Box < dyn HkdfExpander > , inner : KeyScheduleSuite , }
};
}
