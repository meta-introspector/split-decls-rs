macro_rules! deps {
    () => {
        ExtendedCommand!();
    };
}

macro_rules! Command {
    () => {
        deps!();
        # [doc = " The commands are sent by the service control manager to the service through the closure or callback"] # [doc = " passed to the service `run` method."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum Command { # [doc = " The start command is sent when the service first starts."] Start , # [doc = " The stop command is sent when the service is stopping just prior to process termination."] # [doc = ""] # [doc = " This command will only be sent if the `can_stop` method is called as part of construction."] Stop , # [doc = " The pause command is sent when the service is being paused but not stopping."] # [doc = ""] # [doc = " This command will only be sent if the `can_pause` method is called as part of construction."] Pause , # [doc = " The resume command is sent when the service is being resumed following a pause."] # [doc = ""] # [doc = " This command will only be sent if the `can_pause` method is called as part of construction."] Resume , # [doc = " An extended command."] # [doc = ""] # [doc = " Specific commands will only be received if the `can_accept` methods is called to specify those"] # [doc = " commands the service accepts."] Extended (ExtendedCommand) , }
    };
}

Command!();