// Generated macro for spawn_virtiofsd (function)
macro_rules! Depcrate_ci_qemuspawn_virtiofsd {
() => {
// Module: crate::ci::qemu
// Provides: {"spawn_virtiofsd"}
// Dependencies: {}
fn spawn_virtiofsd () -> Result < KillChildOnDrop > { let sh = crate :: sh () ? ; sh . create_dir ("shared") ? ; let cmd = cmd ! (sh , "virtiofsd --socket-path=./vhostqemu --shared-dir ./shared --announce-submounts --sandbox none --seccomp none --inode-file-handles=never") ; eprintln ! ("$ {cmd}") ; Ok (KillChildOnDrop (Command :: from (cmd) . spawn () ?)) }
};
}
