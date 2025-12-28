macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! ExtendedCommand {
    () => {
        deps!();
        # [doc = " A command not specifically covered by the `Command` enum."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub struct ExtendedCommand { # [doc = " The control code for the command."] pub control : u32 , # [doc = " The event type, if any."] pub ty : u32 , # [doc = " The event data, if any."] pub data : * const c_void , }
    };
}

ExtendedCommand!();