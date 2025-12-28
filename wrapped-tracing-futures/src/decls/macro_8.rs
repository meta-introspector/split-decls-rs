macro_rules! deps {
    () => {
        WithDispatch!();
    };
}

macro_rules! macro_8 {
    () => {
        deps!();
        # [cfg (all (feature = "std" , feature = "std-future"))] pin_project ! { # [doc = " A future, stream, sink, or executor that has been instrumented with a"] # [doc = " `tracing` subscriber."] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] # [derive (Clone , Debug)] pub struct WithDispatch < T > { # [pin] inner : T , dispatch : Dispatch , } }
    };
}

macro_8!()