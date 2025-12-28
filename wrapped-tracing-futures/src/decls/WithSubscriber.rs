macro_rules! deps {
    () => {
        WithDispatch!();
    };
}

macro_rules! WithSubscriber {
    () => {
        deps!();
        # [doc = " Extension trait allowing futures, streams, and sinks to be instrumented with"] # [doc = " a `tracing` [`Subscriber`]."] # [doc = ""] # [doc = " [`Subscriber`]: tracing::Subscriber"] # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub trait WithSubscriber : Sized { # [doc = " Attaches the provided [`Subscriber`] to this type, returning a"] # [doc = " `WithDispatch` wrapper."] # [doc = ""] # [doc = " When the wrapped type is a future, stream, or sink, the attached"] # [doc = " subscriber will be set as the [default] while it is being polled."] # [doc = " When the wrapped type is an executor, the subscriber will be set as the"] # [doc = " default for any futures spawned on that executor."] # [doc = ""] # [doc = " [`Subscriber`]: tracing::Subscriber"] # [doc = " [default]: tracing::dispatcher#setting-the-default-subscriber"] fn with_subscriber < S > (self , subscriber : S) -> WithDispatch < Self > where S : Into < Dispatch > , { WithDispatch { inner : self , dispatch : subscriber . into () , } } # [doc = " Attaches the current [default] [`Subscriber`] to this type, returning a"] # [doc = " `WithDispatch` wrapper."] # [doc = ""] # [doc = " When the wrapped type is a future, stream, or sink, the attached"] # [doc = " subscriber will be set as the [default] while it is being polled."] # [doc = " When the wrapped type is an executor, the subscriber will be set as the"] # [doc = " default for any futures spawned on that executor."] # [doc = ""] # [doc = " This can be used to propagate the current dispatcher context when"] # [doc = " spawning a new future."] # [doc = ""] # [doc = " [`Subscriber`]: tracing::Subscriber"] # [doc = " [default]: tracing::dispatcher#setting-the-default-subscriber"] # [inline] fn with_current_subscriber (self) -> WithDispatch < Self > { WithDispatch { inner : self , dispatch : dispatcher :: get_default (| default | default . clone ()) , } } }
    };
}

WithSubscriber!();