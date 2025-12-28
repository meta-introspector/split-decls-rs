macro_rules! deps {
    () => {
        MakeVisitor!();
    };
}

macro_rules! Delimited {
    () => {
        deps!();
        # [doc = " A `MakeVisitor` wrapper that wraps a visitor that writes formatted output so"] # [doc = " that a delimiter is inserted between writing formatted field values."] # [derive (Debug , Clone)] pub struct Delimited < D , V > { delimiter : D , inner : V , }
    };
}

Delimited!();