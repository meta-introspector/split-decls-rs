// Generated macro for print_time_passes_entry (function)
macro_rules! Depcrate_profilingprint_time_passes_entry {
() => {
// Module: crate::profiling
// Provides: {"print_time_passes_entry"}
// Dependencies: {}
pub fn print_time_passes_entry (what : & str , dur : Duration , start_rss : Option < usize > , end_rss : Option < usize > , format : TimePassesFormat ,) { match format { TimePassesFormat :: Json => { let entry = JsonTimePassesEntry { pass : what , time : dur . as_secs_f64 () , start_rss , end_rss } ; eprintln ! (r#"time: {entry}"#) ; return ; } TimePassesFormat :: Text => () , } let is_notable = | | { if dur . as_millis () > 5 { return true ; } if let (Some (start_rss) , Some (end_rss)) = (start_rss , end_rss) { let change_rss = end_rss . abs_diff (start_rss) ; if change_rss > 0 { return true ; } } false } ; if ! is_notable () { return ; } let rss_to_mb = | rss | (rss as f64 / 1_000_000.0) . round () as usize ; let rss_change_to_mb = | rss | (rss as f64 / 1_000_000.0) . round () as i128 ; let mem_string = match (start_rss , end_rss) { (Some (start_rss) , Some (end_rss)) => { let change_rss = end_rss as i128 - start_rss as i128 ; format ! ("; rss: {:>4}MB -> {:>4}MB ({:>+5}MB)" , rss_to_mb (start_rss) , rss_to_mb (end_rss) , rss_change_to_mb (change_rss) ,) } (Some (start_rss) , None) => format ! ("; rss start: {:>4}MB" , rss_to_mb (start_rss)) , (None , Some (end_rss)) => format ! ("; rss end: {:>4}MB" , rss_to_mb (end_rss)) , (None , None) => String :: new () , } ; eprintln ! ("time: {:>7}{}\t{}" , duration_to_secs_str (dur) , mem_string , what) ; }
};
}
