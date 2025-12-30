// Generated macro for handle_ioexit (function)
macro_rules! Depcrate_hypervisorhandle_ioexit {
() => {
// Module: crate::hypervisor
// Provides: {"handle_ioexit"}
// Dependencies: {}
pub (crate) fn handle_ioexit (meta : & X86TestFn , cpu : & mut Vcpu , run : & kvm :: Run , printer : & mut SerialPrinter ,) -> Result < IoHandleStatus , IoHandleError > { let io = unsafe { * run . io () } ; match io . direction { IoDirection :: In => { let mut regs = cpu . get_regs () . unwrap () ; if io . port == 0x3fd { regs . rax = 0x20 ; cpu . set_regs (& regs) . unwrap () ; return Ok (IoHandleStatus :: Handled) ; } else if io . port == 0x2fd { regs . rax = 0x20 ; cpu . set_regs (& regs) . unwrap () ; return Ok (IoHandleStatus :: Handled) ; } else if io . port == meta . ioport_enable . 0 { regs . rax = meta . ioport_enable . 1 as u64 ; cpu . set_regs (& regs) . unwrap () ; return Ok (IoHandleStatus :: Handled) ; } return Err (IoHandleError :: UnexpectedRead (io . port)) ; } IoDirection :: Out => { let regs = cpu . get_regs () . unwrap () ; if io . port == 0x3f8 { printer . write (& [regs . rax as u8]) . ok () ; return Ok (IoHandleStatus :: Handled) ; } else if io . port == 0x2f8 { return Ok (IoHandleStatus :: Handled) ; } else if io . port == 0xf4 && regs . rax as u8 == 0x0 { return Ok (IoHandleStatus :: TestSuccessful) ; } else if io . port == meta . ioport_enable . 0 && regs . rax == meta . ioport_enable . 1 as u64 { return Ok (IoHandleStatus :: Handled) ; } else if io . port == 0xf4 { return Ok (IoHandleStatus :: TestPanic (regs . rax as u8)) ; } return Err (IoHandleError :: UnexpectedWrite (io . port , regs . rax as u32)) ; } } ; }
};
}
