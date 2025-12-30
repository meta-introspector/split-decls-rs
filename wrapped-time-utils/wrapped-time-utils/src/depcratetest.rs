// Generated macro for test (module)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_interval_update () { let i = AtomicInterval :: default () ; assert ! (! i . should_update (1000)) ; let i = AtomicInterval :: default () ; assert ! (i . should_update_ext (1000 , false)) ; std :: thread :: sleep (Duration :: from_millis (10)) ; assert ! (i . elapsed_ms () > 9 && i . elapsed_ms () < 1000) ; assert ! (i . remaining_until_next_interval (1000) > 9 && i . remaining_until_next_interval (1000) < 991) ; assert ! (i . should_update (9)) ; assert ! (! i . should_update (100)) ; } # [test] fn test_years_as_slots () { let tick_duration = Duration :: from_micros (1000 * 1000 / 160) ; assert_eq ! (years_as_slots (0.0 , & tick_duration , 4) as u64 , 0) ; assert_eq ! (years_as_slots (1.0 / 12f64 , & tick_duration , 4) as u64 , 105_189_753) ; assert_eq ! (years_as_slots (1.0 , & tick_duration , 4) as u64 , 1_262_277_039) ; let tick_duration = Duration :: from_micros (1000 * 1000) ; assert_eq ! (years_as_slots (1.0 / SECONDS_PER_YEAR , & tick_duration , 1) , 1.0) ; } # [test] fn test_slot_duration_from_slots_per_year () { let slots_per_year = 1_262_277_039.0 ; let ticks_per_slot = 4 ; assert_eq ! (slot_duration_from_slots_per_year (slots_per_year) , Duration :: from_micros (1000 * 1000 / 160) * ticks_per_slot) ; assert_eq ! (slot_duration_from_slots_per_year (0.0) , Duration :: from_micros (0) * ticks_per_slot) ; let slots_per_year = SECONDS_PER_YEAR ; let ticks_per_slot = 1 ; assert_eq ! (slot_duration_from_slots_per_year (slots_per_year) , Duration :: from_millis (1000) * ticks_per_slot) ; } }
};
}
