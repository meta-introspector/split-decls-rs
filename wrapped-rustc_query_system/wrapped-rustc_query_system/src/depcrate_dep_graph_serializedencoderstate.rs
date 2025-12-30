// Generated macro for EncoderState (struct)
macro_rules! Depcrate_dep_graph_serializedEncoderState {
() => {
// Module: crate::dep_graph::serialized
// Provides: {"EncoderState"}
// Dependencies: {}
struct EncoderState < D : Deps > { next_node_index : AtomicU64 , previous : Arc < SerializedDepGraph > , file : Lock < Option < FileEncoder > > , local : WorkerLocal < RefCell < LocalEncoderState > > , stats : Option < Lock < FxHashMap < DepKind , Stat > > > , marker : PhantomData < D > , }
};
}
