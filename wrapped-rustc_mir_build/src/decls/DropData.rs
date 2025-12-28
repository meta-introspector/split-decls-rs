macro_rules! deps {
    () => {
        DropKind!();
    };
}

macro_rules! DropData {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug)] struct DropData { # [doc = " The `Span` where drop obligation was incurred (typically where place was"] # [doc = " declared)"] source_info : SourceInfo , # [doc = " local to drop"] local : Local , # [doc = " Whether this is a value Drop or a StorageDead."] kind : DropKind , }
    };
}

DropData!()