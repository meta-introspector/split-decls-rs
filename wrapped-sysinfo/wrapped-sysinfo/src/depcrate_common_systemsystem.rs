// Generated macro for System (struct)
macro_rules! Depcrate_common_systemSystem {
() => {
// Module: crate::common::system
// Provides: {"System"}
// Dependencies: {}
# [doc = " Type containing system's information such as processes, memory and CPU."] # [doc = ""] # [doc = " ⚠\u{fe0f} On newer Android versions, there are restrictions on which system information"] # [doc = " a non-system application has access to. So CPU information might not be available."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::System;"] # [doc = ""] # [doc = " if sysinfo::IS_SUPPORTED_SYSTEM {"] # [doc = "     println!(\"System: {:?}\", System::new_all());"] # [doc = " } else {"] # [doc = "     println!(\"This OS isn't supported (yet?).\");"] # [doc = " }"] # [doc = " ```"] pub struct System { pub (crate) inner : SystemInner , }
};
}
