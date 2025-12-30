// Generated macro for WriteLrat (struct)
macro_rules! DepcrateWriteLrat {
() => {
// Module: crate
// Provides: {"WriteLrat"}
// Dependencies: {}
# [doc = " Proof processor that generates an LRAT proof."] pub struct WriteLrat < 'a > { binary : bool , target : BufWriter < Box < dyn Write + 'a > > , delete_open : bool , last_added_id : u64 , buffered_deletes : Vec < u64 > , }
};
}
