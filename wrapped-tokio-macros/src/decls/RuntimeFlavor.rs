macro_rules! RuntimeFlavor {
    () => {
        # [derive (Clone , Copy , PartialEq)] enum RuntimeFlavor { CurrentThread , Threaded , Local , }
    };
}

RuntimeFlavor!();