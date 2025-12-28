macro_rules! deps {
    () => {
        ProxyData!();
    };
}

macro_rules! Proxy {
    () => {
        deps!();
        # [doc = " This is a jobserver proxy used to ensure that we hold on to at least one token."] pub struct Proxy { client : Client , data : Mutex < ProxyData > , # [doc = " Threads which are waiting on a token will wait on this."] wake_pending : Condvar , helper : OnceLock < HelperThread > , }
    };
}

Proxy!();