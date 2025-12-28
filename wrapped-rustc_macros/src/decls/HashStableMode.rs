macro_rules! HashStableMode {
    () => {
        enum HashStableMode { Normal , Generic , NoContext , }
    };
}

HashStableMode!()