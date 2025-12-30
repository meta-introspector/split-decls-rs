// Generated macro for impl_91 (impl)
macro_rules! Depcrate_dataimpl_91 {
() => {
// Module: crate::data
// Provides: {"impl_91"}
// Dependencies: {}
impl GlobalSchemes { pub fn try_from_raw (raw : u8) -> Option < Self > { match raw { 1 => Some (Self :: Debug) , 2 => Some (Self :: Event) , 3 => Some (Self :: Memory) , 4 => Some (Self :: Pipe) , 5 => Some (Self :: Serio) , 6 => Some (Self :: Irq) , 7 => Some (Self :: Time) , 8 => Some (Self :: Sys) , 9 => Some (Self :: Proc) , 10 => Some (Self :: Acpi) , 11 => Some (Self :: Dtb) , _ => None , } } pub fn as_str (& self) -> & 'static str { match self { Self :: Debug => "debug" , Self :: Event => "event" , Self :: Memory => "memory" , Self :: Pipe => "pipe" , Self :: Serio => "serio" , Self :: Irq => "irq" , Self :: Time => "time" , Self :: Sys => "sys" , Self :: Proc => "kernel.proc" , Self :: Acpi => "kernel.acpi" , Self :: Dtb => "kernel.dtb" , } } }
};
}
