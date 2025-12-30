// Generated macro for Command (enum)
macro_rules! DepcrateCommand {
() => {
// Module: crate
// Provides: {"Command"}
// Dependencies: {}
# [derive (Debug , Parser)] enum Command { # [doc = " Run the gRPC server"] Server , # [doc = " Get the value at some key"] Get { # [arg (short = 'k' , long)] key : String , } , # [doc = " Set a value at some key."] # [doc = ""] # [doc = " The value will be read from stdin."] Set { # [arg (short = 'k' , long)] key : String , } , # [doc = " Subscribe to a stream of inserted keys"] Subscribe , }
};
}
