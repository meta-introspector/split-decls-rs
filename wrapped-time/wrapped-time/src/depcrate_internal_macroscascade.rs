// Generated macro for cascade (macro)
macro_rules! Depcrate_internal_macroscascade {
() => {
// Module: crate::internal_macros
// Provides: {"cascade"}
// Dependencies: {}
# [doc = " Cascade an out-of-bounds value."] macro_rules ! cascade { (@ ordinal ordinal) => { } ; (@ year year) => { } ; ($ from : ident in $ min : literal .. $ max : expr => $ to : tt) => { # [allow (unused_comparisons , unused_assignments)] let min = $ min ; let max = $ max ; if crate :: hint :: unlikely ($ from >= max) { $ from -= max - min ; $ to += 1 ; } else if crate :: hint :: unlikely ($ from < min) { $ from += max - min ; $ to -= 1 ; } } ; ($ ordinal : ident => $ year : ident) => { cascade ! (@ ordinal $ ordinal) ; cascade ! (@ year $ year) ; let days_in_year = crate :: util :: days_in_year ($ year) as i16 ; # [allow (unused_assignments)] if crate :: hint :: unlikely ($ ordinal > days_in_year) { $ ordinal -= days_in_year ; $ year += 1 ; } else if crate :: hint :: unlikely ($ ordinal < 1) { $ year -= 1 ; $ ordinal += crate :: util :: days_in_year ($ year) as i16 ; } } ; }
};
}
