// Generated macro for write_varisat_step (function)
macro_rules! Depcrate_proofwrite_varisat_step {
() => {
// Module: crate::proof
// Provides: {"write_varisat_step"}
// Dependencies: {}
# [doc = " Write a step using our native format"] fn write_varisat_step < 'a , 's > (mut ctx : partial ! (Context <'a >, mut ProofP <'a >, SolverStateP) , map_vars : impl Fn (Var) -> Var , step : & 's ProofStep < 's > ,) -> io :: Result < () > { let (proof , ctx) = ctx . split_part_mut (ProofP) ; proof . clause_count += clause_count_delta (step) ; let mut rehash = false ; while proof . clause_count > (1 << (proof . hash_bits / 2)) { proof . hash_bits += 2 ; rehash = true ; } if ctx . part (SolverStateP) . solver_invoked { while proof . hash_bits > 6 && proof . clause_count * 4 < (1 << (proof . hash_bits / 2)) { proof . hash_bits -= 2 ; rehash = true ; } } if rehash { varisat_internal_proof :: binary_format :: write_step (& mut proof . target , & ProofStep :: ChangeHashBits { bits : proof . hash_bits , } ,) ? ; } let shift_bits = ClauseHash :: max_value () . count_ones () - proof . hash_bits ; let map_hash = | hash | hash >> shift_bits ; let step = proof . map_step . map (step , map_vars , map_hash) ; if proof . format == Some (ProofFormat :: Varisat) { varisat_internal_proof :: binary_format :: write_step (& mut proof . target , & step) ? ; } Ok (()) }
};
}
