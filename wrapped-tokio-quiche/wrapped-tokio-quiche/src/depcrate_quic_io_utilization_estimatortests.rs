// Generated macro for tests (module)
macro_rules! Depcrate_quic_io_utilization_estimatortests {
() => {
// Module: crate::quic::io::utilization_estimator
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn estimate () { let mut now = Instant :: now () ; let mut estimator = MaxUtilizedBandwidthEstimator :: new () ; assert_eq ! (estimator . get () . bandwidth , 0) ; assert ! (estimator . estimate . get_best () . is_none ()) ; estimator . new_round (now , 30_000_000 , 0 , 0) ; assert_eq ! (estimator . get () . bandwidth , 0) ; assert ! (estimator . estimate . get_best () . is_none ()) ; now += Duration :: from_secs (30) ; estimator . new_round (now , 60_000_000 , 0 , 30_000_000) ; assert_eq ! (estimator . get () . bandwidth , 0) ; assert_eq ! (estimator . estimate . get_best () . unwrap () . bandwidth , 8_000_000) ; now += Duration :: from_secs (30) ; estimator . new_round (now , 90_000_000 , 0 , 60_000_000) ; assert_eq ! (estimator . get () . bandwidth , 0) ; assert_eq ! (estimator . estimate . get_best () . unwrap () . bandwidth , 12_000_000) ; now += Duration :: from_secs (30) ; estimator . new_round (now , 30_000_000 , 0 , 90_000_000) ; assert_eq ! (estimator . get () . bandwidth , 0) ; assert_eq ! (estimator . estimate . get_best () . unwrap () . bandwidth , 16_000_000) ; for _ in 0 .. 4 { now += Duration :: from_secs (30) ; estimator . new_round (now , 30_000_000 , 0 , 30_000_000) ; assert_eq ! (estimator . get () . bandwidth , 16_000_000) ; } now += Duration :: from_secs (30) ; estimator . new_round (now , 30_000_000 , 0 , 30_000_000) ; assert ! (estimator . get () . bandwidth < 8 * 2_000_000) ; } }
};
}
