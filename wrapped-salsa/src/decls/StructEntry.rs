macro_rules! deps {
    () => {
        Value!();
        Configuration!();
        DatabaseKeyIndex!();
    };
}

macro_rules! StructEntry {
    () => {
        deps!();
        # [doc = " A tracked struct entry."] pub struct StructEntry < 'db , C > where C : Configuration , { value : & 'db Value < C > , key : DatabaseKeyIndex , }
    };
}

StructEntry!()