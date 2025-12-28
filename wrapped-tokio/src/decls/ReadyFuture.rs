macro_rules! ReadyFuture {
    () => {
        type ReadyFuture < T > = future :: Ready < io :: Result < T > > ;
    };
}

ReadyFuture!()