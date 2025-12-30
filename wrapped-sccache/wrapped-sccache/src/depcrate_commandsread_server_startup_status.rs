// Generated macro for read_server_startup_status (function)
macro_rules! Depcrate_commandsread_server_startup_status {
() => {
// Module: crate::commands
// Provides: {"read_server_startup_status"}
// Dependencies: {}
async fn read_server_startup_status < R : AsyncReadExt + Unpin > (mut server : R ,) -> Result < ServerStartup > { let mut bytes = [0u8 ; 4] ; server . read_exact (& mut bytes [..]) . await ? ; let len = BigEndian :: read_u32 (& bytes) ; let mut data = vec ! [0 ; len as usize] ; server . read_exact (data . as_mut_slice ()) . await ? ; Ok (bincode :: deserialize (& data) ?) }
};
}
