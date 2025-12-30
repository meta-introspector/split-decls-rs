// Generated macro for JobOwner (struct)
macro_rules! Depcrate_query_plumbingJobOwner {
() => {
// Module: crate::query::plumbing
// Provides: {"JobOwner"}
// Dependencies: {}
# [doc = " A type representing the responsibility to execute the job in the `job` field."] # [doc = " This will poison the relevant query if dropped."] struct JobOwner < 'tcx , K , I > where K : Eq + Hash + Copy , { state : & 'tcx QueryState < K , I > , key : K , }
};
}
