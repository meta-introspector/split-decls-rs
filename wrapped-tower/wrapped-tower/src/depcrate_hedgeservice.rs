// Generated macro for Service (type)
macro_rules! Depcrate_hedgeService {
() => {
// Module: crate::hedge
// Provides: {"Service"}
// Dependencies: {}
type Service < S , P > = select :: Select < SelectPolicy < P > , Latency < Histo , S > , Delay < DelayPolicy , AsyncFilter < Latency < Histo , S > , PolicyPredicate < P > > > , > ;
};
}
