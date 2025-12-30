// Generated macro for conditional_pigeon_hole (function)
macro_rules! Depcrate_testconditional_pigeon_hole {
() => {
// Module: crate::test
// Provides: {"conditional_pigeon_hole"}
// Dependencies: {}
# [doc = " Generates a conditional pigeon hole principle formula."] pub fn conditional_pigeon_hole (columns : impl Strategy < Value = usize > , extra_rows : impl Strategy < Value = usize > ,) -> impl Strategy < Value = (Vec < Lit > , usize , CnfFormula) > { (columns , extra_rows) . prop_flat_map (| (columns , extra_rows) | { let rows = columns + extra_rows ; let vars = (columns + 1) * rows ; collection :: vec (bool :: ANY , vars) . prop_perturb (move | polarity , mut rng | { let mut clauses : Vec < Vec < Lit > > = vec ! [] ; let lits = polarity . into_iter () . enumerate () . map (| (index , polarity) | Lit :: from_index (index , polarity)) . collect :: < Vec < _ > > () ; for i in 1 .. columns + 1 { for j in 0 .. rows { for k in 0 .. j { let mut clause = [lits [i * rows + j] , lits [i * rows + k]] ; clause . shuffle (& mut rng) ; clauses . push (clause [..] . to_owned ()) ; } } } for j in 0 .. rows { let mut clause : Vec < _ > = (0 .. columns + 1) . map (| i | ! lits [i * rows + j]) . collect () ; clause . shuffle (& mut rng) ; clauses . push (clause [..] . to_owned ()) ; } clauses . shuffle (& mut rng) ; (lits [0 .. rows] . to_owned () , columns , CnfFormula :: from (clauses)) }) }) }
};
}
