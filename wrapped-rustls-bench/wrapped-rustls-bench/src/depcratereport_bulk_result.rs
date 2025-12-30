// Generated macro for report_bulk_result (function)
macro_rules! Depcratereport_bulk_result {
() => {
// Module: crate
// Provides: {"report_bulk_result"}
// Dependencies: {}
fn report_bulk_result (variant : & str , params : & Parameters , timings : Vec < Timings > , rounds : u64) { let mfs_str = format ! ("max_fragment_size:{}" , params . max_fragment_size . map (| v | v . to_string ()) . unwrap_or_else (|| "default" . to_string ())) ; let total_mbs = ((params . plaintext_size * rounds) as f64) / (1024. * 1024.) ; print ! ("{}\t{:?}\t{:?}\t{}\tsend\t" , variant , params . proto . version , params . proto . ciphersuite , mfs_str ,) ; report_timings ("MB/s" , & timings , total_mbs , | t | t . server) ; print ! ("{}\t{:?}\t{:?}\t{}\trecv\t" , variant , params . proto . version , params . proto . ciphersuite , mfs_str ,) ; report_timings ("MB/s" , & timings , total_mbs , | t | t . client) ; }
};
}
