macro_rules! deps {
    () => {
        ThreadWaker!();
    };
}

macro_rules! MockTask {
    () => {
        deps!();
        # [derive (Debug , Clone)] struct MockTask { waker : Arc < ThreadWaker > , }
    };
}

MockTask!()