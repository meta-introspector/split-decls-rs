// Generated macro for TestEnvironment (struct)
macro_rules! Depcrate_hypervisorTestEnvironment {
() => {
// Module: crate::hypervisor
// Provides: {"TestEnvironment"}
// Dependencies: {}
pub (crate) struct TestEnvironment < 'a > { sys : & 'a System , # [allow (unused)] heap : & 'a mut PhysicalMemory , stack : & 'a mut PhysicalMemory , vspace : VSpace < 'a > , vm : VirtualMachine < 'a > , }
};
}
