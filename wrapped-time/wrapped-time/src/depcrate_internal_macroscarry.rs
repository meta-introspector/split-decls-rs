// Generated macro for carry (macro)
macro_rules! Depcrate_internal_macroscarry {
() => {
// Module: crate::internal_macros
// Provides: {"carry"}
// Dependencies: {}
# [doc = " Similar to `overflowing_add`, but returning the number of times that it overflowed. Contained to"] # [doc = " a certain range and only overflows a maximum number of times."] macro_rules ! carry { (@ most_once $ value : expr , $ min : literal .. $ max : expr) => { match ($ value , $ min , $ max) { (value , min , max) => { if crate :: hint :: likely (value >= min) { if crate :: hint :: likely (value < max) { (value , 0) } else { (value - (max - min) , 1) } } else { (value + (max - min) , - 1) } } } } ; (@ most_twice $ value : expr , $ min : literal .. $ max : expr) => { match ($ value , $ min , $ max) { (value , min , max) => { if crate :: hint :: likely (value >= min) { if crate :: hint :: likely (value < max) { (value , 0) } else if value < 2 * max - min { (value - (max - min) , 1) } else { (value - 2 * (max - min) , 2) } } else { if value >= min - max { (value + (max - min) , - 1) } else { (value + 2 * (max - min) , - 2) } } } } } ; (@ most_thrice $ value : expr , $ min : literal .. $ max : expr) => { match ($ value , $ min , $ max) { (value , min , max) => { if crate :: hint :: likely (value >= min) { if crate :: hint :: likely (value < max) { (value , 0) } else if value < 2 * max - min { (value - (max - min) , 1) } else if value < 3 * max - 2 * min { (value - 2 * (max - min) , 2) } else { (value - 3 * (max - min) , 3) } } else { if value >= min - max { (value + (max - min) , - 1) } else if value >= 2 * (min - max) { (value + 2 * (max - min) , - 2) } else { (value + 3 * (max - min) , - 3) } } } } } ; }
};
}
