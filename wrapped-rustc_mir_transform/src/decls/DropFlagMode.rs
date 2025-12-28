macro_rules! DropFlagMode {
    () => {
        # [doc = " Which drop flags to affect/check with an operation."] # [derive (Debug)] pub (crate) enum DropFlagMode { # [doc = " Only affect the top-level drop flag, not that of any contained fields."] Shallow , # [doc = " Affect all nested drop flags in addition to the top-level one."] Deep , }
    };
}

DropFlagMode!()