macro_rules! RecordType {
    () => {
        # [doc = " Indicates whether a field should be recorded as `Value` or `Debug`."] enum RecordType { # [doc = " The field should be recorded using its `Value` implementation."] Value , # [doc = " The field should be recorded using `tracing::field::debug()`."] Debug , }
    };
}

RecordType!()