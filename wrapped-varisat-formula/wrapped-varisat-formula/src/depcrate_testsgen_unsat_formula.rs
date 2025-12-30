// Generated macro for sgen_unsat_formula (function)
macro_rules! Depcrate_testsgen_unsat_formula {
() => {
// Module: crate::test
// Provides: {"sgen_unsat_formula"}
// Dependencies: {}
# [doc = " Generate small hard unsat instances."] # [doc = ""] # [doc = " Implementation of http://www.cs.qub.ac.uk/~i.spence/sgen/ but with random partitions"] pub fn sgen_unsat_formula (blocks : impl Strategy < Value = usize > ,) -> impl Strategy < Value = CnfFormula > { blocks . prop_flat_map (| blocks | { collection :: vec (bool :: ANY , blocks * 4 + 1) . prop_perturb (| polarity , mut rng | { let mut clauses : Vec < Vec < Lit > > = vec ! [] ; let mut lits = polarity . into_iter () . enumerate () . map (| (index , polarity) | Lit :: from_index (index , polarity)) . collect :: < Vec < _ > > () ; for & invert in [false , true] . iter () { lits . shuffle (& mut rng) ; for block in lits . chunks_exact (4) { for a in 0 .. 4 { for b in 0 .. a { for c in 0 .. b { let mut clause = vec ! [block [a] ^ invert , block [b] ^ invert , block [c] ^ invert] ; clause . shuffle (& mut rng) ; clauses . push (clause) ; } } } } let & lit_a = lits . last () . unwrap () ; for b in 0 .. 4 { for c in 0 .. b { let mut clause = vec ! [lit_a ^ invert , lits [b] ^ invert , lits [c] ^ invert] ; clause . shuffle (& mut rng) ; clauses . push (clause) ; } } } clauses . shuffle (& mut rng) ; CnfFormula :: from (clauses) }) }) }
};
}
