// Generated macro for aggregate (function)
macro_rules! Depcrateaggregate {
() => {
// Module: crate
// Provides: {"aggregate"}
// Dependencies: {}
fn aggregate (opt : AggregateOpt) -> Result < () , Box < dyn Error + Send + Sync > > { let profiles = opt . files . into_iter () . map (| file | ProfilingData :: new (& file)) . collect :: < Result < Vec < _ > , _ > > () ? ; aggregate :: aggregate_profiles (profiles) ; Ok (()) }
};
}
