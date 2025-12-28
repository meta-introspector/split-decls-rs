macro_rules! TraceStyle {
    () => {
        # [doc = " Decides how traces will be recorded."] # [derive (Default)] pub enum TraceStyle { # [doc = " Traces will be recorded as a group of threads."] # [doc = " In this style, spans should be entered and exited on the same thread."] # [default] Threaded , # [doc = " Traces will recorded as a group of asynchronous operations."] Async , }
    };
}

TraceStyle!();