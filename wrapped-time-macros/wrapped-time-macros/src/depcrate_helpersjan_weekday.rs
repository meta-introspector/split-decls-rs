// Generated macro for jan_weekday (function)
macro_rules! Depcrate_helpersjan_weekday {
() => {
// Module: crate::helpers
// Provides: {"jan_weekday"}
// Dependencies: {}
fn jan_weekday (year : i32 , ordinal : i32) -> u8 { macro_rules ! div_floor { ($ a : expr , $ b : expr) => { { let (_quotient , _remainder) = ($ a / $ b , $ a % $ b) ; if (_remainder > 0 && $ b < 0) || (_remainder < 0 && $ b > 0) { _quotient - 1 } else { _quotient } } } ; } let adj_year = year - 1 ; (ordinal + adj_year + div_floor ! (adj_year , 4) - div_floor ! (adj_year , 100) + div_floor ! (adj_year , 400) + 6) . rem_euclid (7) . cast_unsigned () . truncate () }
};
}
