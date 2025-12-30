// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () , Box < dyn Error + Send + Sync > > { let opt = Opt :: parse () ; let profiling_data = ProfilingData :: new (& opt . file_prefix) ? ; let recorded_stacks = collapse_stacks (& profiling_data) ; let mut file = BufWriter :: new (File :: create ("out.stacks_folded") ?) ; for (unique_stack , count) in recorded_stacks { writeln ! (file , "{} {}" , unique_stack , count) ? ; } Ok (()) }
};
}
