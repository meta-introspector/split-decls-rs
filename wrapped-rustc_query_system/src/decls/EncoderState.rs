macro_rules! deps {
    () => {
        Deps!();
        Stat!();
        SerializedDepGraph!();
        LocalEncoderState!();
        DepKind!();
    };
}

macro_rules! EncoderState {
    () => {
        deps!();
        struct EncoderState < D : Deps > { next_node_index : AtomicU64 , previous : Arc < SerializedDepGraph > , file : Lock < Option < FileEncoder > > , local : WorkerLocal < RefCell < LocalEncoderState > > , stats : Option < Lock < FxHashMap < DepKind , Stat > > > , marker : PhantomData < D > , }
    };
}

EncoderState!();