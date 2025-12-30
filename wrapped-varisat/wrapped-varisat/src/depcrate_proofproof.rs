// Generated macro for Proof (struct)
macro_rules! Depcrate_proofProof {
() => {
// Module: crate::proof
// Provides: {"Proof"}
// Dependencies: {}
# [doc = " Proof generation."] pub struct Proof < 'a > { format : Option < ProofFormat > , target : BufWriter < Box < dyn Write + 'a > > , checker : Option < Checker < 'a > > , map_step : map_step :: MapStep , # [doc = " How many bits are used for storing clause hashes."] hash_bits : u32 , # [doc = " How many clauses are currently in the db."] # [doc = ""] # [doc = " This is used to pick a good number of hash_bits"] clause_count : isize , }
};
}
