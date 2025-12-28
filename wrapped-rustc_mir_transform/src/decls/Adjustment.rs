macro_rules! deps {
    () => {
        DerefSource!();
    };
}

macro_rules! Adjustment {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , PartialEq)] enum Adjustment { # [doc = " Pass the receiver as-is."] Identity , # [doc = " We get passed a reference or a raw pointer to `self` and call the target with `*self`."] # [doc = ""] # [doc = " This either copies `self` (if `Self: Copy`, eg. for function items), or moves out of it"] # [doc = " (for `VTableShim`, which effectively is passed `&own Self`)."] Deref { source : DerefSource } , # [doc = " We get passed `self: Self` and call the target with `&mut self`."] # [doc = ""] # [doc = " In this case we need to ensure that the `Self` is dropped after the call, as the callee"] # [doc = " won't do it for us."] RefMut , }
    };
}

Adjustment!();