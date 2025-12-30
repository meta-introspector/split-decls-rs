// Generated macro for process_split_io (function)
macro_rules! Depcrate_cmdprocess_split_io {
() => {
// Module: crate::cmd
// Provides: {"process_split_io"}
// Dependencies: {}
fn process_split_io (child : & mut std :: process :: Child , input : Option < Vec < u8 > > ,) -> std :: io :: Result < (Option < Stream > , Option < Stream >) > { use std :: io :: Write ; let stdin = input . and_then (| i | { child . stdin . take () . map (| mut stdin | std :: thread :: spawn (move | | stdin . write_all (& i))) }) ; let stdout = child . stdout . take () . map (threaded_read) ; let stderr = child . stderr . take () . map (threaded_read) ; stdin . and_then (| t | t . join () . unwrap () . ok ()) ; Ok ((stdout , stderr)) }
};
}
