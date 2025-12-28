macro_rules! Uniform {
    () => {
        # [doc = " An argument passed entirely registers with the"] # [doc = " same kind (e.g., HFA / HVA on PPC64 and AArch64)."] # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug , HashStable_Generic)] pub struct Uniform { pub unit : Reg , # [doc = " The total size of the argument, which can be:"] # [doc = " * equal to `unit.size` (one scalar/vector),"] # [doc = " * a multiple of `unit.size` (an array of scalar/vectors),"] # [doc = " * if `unit.kind` is `Integer`, the last element can be shorter, i.e., `{ i64, i64, i32 }`"] # [doc = "   for 64-bit integers with a total size of 20 bytes. When the argument is actually passed,"] # [doc = "   this size will be rounded up to the nearest multiple of `unit.size`."] pub total : Size , # [doc = " Indicate that the argument is consecutive, in the sense that either all values need to be"] # [doc = " passed in register, or all on the stack. If they are passed on the stack, there should be"] # [doc = " no additional padding between elements."] pub is_consecutive : bool , }
    };
}

Uniform!();