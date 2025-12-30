// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let table = [SERVICE_TABLE_ENTRYW { lpServiceName : & mut 0 , lpServiceProc : Some (service_main) , } , SERVICE_TABLE_ENTRYW :: default () ,] ; unsafe { if StartServiceCtrlDispatcherW (table . as_ptr ()) == 0 { println ! (r#"Use Service Control Manager to start service.

Install:
  > sc create rust binPath= "{}"

Start:
  > sc start rust

Query status:
  > sc query rust

Stop:
  > sc stop rust

Delete (uninstall):
  > sc delete rust
"# , std :: env :: current_exe () . unwrap () . display ()) ; } } log ("service exit\n") ; }
};
}
