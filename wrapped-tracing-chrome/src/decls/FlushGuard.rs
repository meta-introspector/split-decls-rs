macro_rules! deps {
    () => {
        Message!();
    };
}

macro_rules! FlushGuard {
    () => {
        deps!();
        # [doc = " This guard will signal the thread writing the trace file to stop and join it when dropped."] pub struct FlushGuard { sender : Sender < Message > , handle : Cell < Option < JoinHandle < () > > > , }
    };
}

FlushGuard!()