macro_rules! deps {
    () => {
        Inner!();
        Builder!();
    };
}

macro_rules! Mock {
    () => {
        deps!();
        # [doc = " An I/O object that follows a predefined script."] # [doc = ""] # [doc = " This value is created by `Builder` and implements `AsyncRead` + `AsyncWrite`. It"] # [doc = " follows the scenario described by the builder and panics otherwise."] # [derive (Debug)] pub struct Mock { inner : Inner , }
    };
}

Mock!()