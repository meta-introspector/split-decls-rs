// Generated macro for X86TestFn (struct)
macro_rules! DepcrateX86TestFn {
() => {
// Module: crate
// Provides: {"X86TestFn"}
// Dependencies: {}
pub struct X86TestFn { # [doc = " Name of test."] pub name : & 'static str , # [doc = " Ignore this test?"] pub ignore : bool , # [doc = " Create an identify map of process inside the VM?"] pub identity_map : bool , # [doc = " Add guest physical memory in this range."] pub physical_memory : (u64 , u64) , # [doc = " When read on ioport_enable.0 return ioport_enable.1 as value."] # [doc = " When write on ioport_enable.0 abort if value was not ioport_enable.1."] pub ioport_enable : (u16 , u32) , # [doc = " Test has a #[should_panic] attribute"] pub should_panic : bool , # [doc = " Test has a #[should_halt] attribute"] pub should_halt : bool , # [doc = " Test function we need to execute (in a VM)."] pub testfn : StaticTestFn , }
};
}
