macro_rules! deps {
    () => {
        QueryJobId!();
    };
}

macro_rules! Waiter {
    () => {
        deps!();
        # [doc = " A resumable waiter of a query. The usize is the index into waiters in the query's latch"] type Waiter = (QueryJobId , usize) ;
    };
}

Waiter!()