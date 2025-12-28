macro_rules! NeedsTemporary {
    () => {
        # [derive (Debug)] enum NeedsTemporary { # [doc = " Use this variant when whatever you are converting with `as_operand`"] # [doc = " is the last thing you are converting. This means that if we introduced"] # [doc = " an intermediate temporary, we'd only read it immediately after, so we can"] # [doc = " also avoid it."] No , # [doc = " For all cases where you aren't sure or that are too expensive to compute"] # [doc = " for now. It is always safe to fall back to this."] Maybe , }
    };
}

NeedsTemporary!()