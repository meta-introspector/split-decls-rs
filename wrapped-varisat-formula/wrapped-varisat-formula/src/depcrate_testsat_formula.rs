// Generated macro for sat_formula (function)
macro_rules! Depcrate_testsat_formula {
() => {
// Module: crate::test
// Provides: {"sat_formula"}
// Dependencies: {}
# [doc = " Generate a sat instance."] # [doc = ""] # [doc = " This generates a random full assignment and then only generates clauses compatible with that"] # [doc = " assignment."] pub fn sat_formula (vars : impl Strategy < Value = usize > , clause_count : impl Strategy < Value = usize > , density : impl Strategy < Value = f64 > , polarity_dist : impl Strategy < Value = f64 > ,) -> impl Strategy < Value = CnfFormula > { (vars , clause_count , density , polarity_dist) . prop_flat_map (| (vars , clause_count , density , polarity_dist) | { let density = Bernoulli :: new (density) . unwrap () ; let polarity_dist = Bernoulli :: new (polarity_dist) . unwrap () ; collection :: vec (bool :: ANY , vars) . prop_perturb (move | polarity , mut rng | { let mut clauses : Vec < Vec < Lit > > = vec ! [] ; let lits = polarity . into_iter () . enumerate () . map (| (index , polarity) | Lit :: from_index (index , polarity)) . collect :: < Vec < _ > > () ; for _ in 0 .. clause_count { let & fixed_lit = lits . choose (& mut rng) . unwrap () ; let mut clause = vec ! [fixed_lit] ; for & lit in lits . iter () { if lit != fixed_lit && rng . sample (density) { clause . push (lit ^ rng . sample (polarity_dist)) ; } } clause . shuffle (& mut rng) ; clauses . push (clause) ; } clauses . shuffle (& mut rng) ; CnfFormula :: from (clauses) }) } ,) }
};
}
