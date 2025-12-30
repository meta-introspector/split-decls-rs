// Generated macro for seq_rule (function)
macro_rules! Depcrate_parserseq_rule {
() => {
// Module: crate::parser
// Provides: {"seq_rule"}
// Dependencies: {}
fn seq_rule (p : & mut Parser) -> Result < Rule > { let lhs = atom_rule (p) ? ; let mut seq = vec ! [lhs] ; while let Some (rule) = opt_atom_rule (p) ? { seq . push (rule) } let res = if seq . len () == 1 { seq . pop () . unwrap () } else { Rule :: Seq (seq) } ; Ok (res) }
};
}
