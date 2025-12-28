macro_rules! LogTracer {
    () => {
        # [doc = " A simple \"logger\" that converts all log records into `tracing` `Event`s."] # [derive (Debug)] pub struct LogTracer { ignore_crates : Box < [String] > , }
    };
}

LogTracer!();