// Generated macro for report_timings (function)
macro_rules! Depcratereport_timings {
() => {
// Module: crate
// Provides: {"report_timings"}
// Dependencies: {}
fn report_timings (units : & str , thread_timings : & [Timings] , work_per_thread : f64 , which : impl Fn (& Timings) -> f64 ,) { if let & [timing] = thread_timings { println ! ("{:.2}\t{}" , work_per_thread / which (& timing) , units) ; return ; } let mut total_rate = 0. ; print ! ("threads\t{}\t" , thread_timings . len ()) ; for t in thread_timings . iter () { let rate = work_per_thread / which (t) ; total_rate += rate ; print ! ("{rate:.2}\t") ; } println ! ("total\t{:.2}\tper-thread\t{:.2}\t{}" , total_rate , total_rate / (thread_timings . len () as f64) , units ,) ; }
};
}
