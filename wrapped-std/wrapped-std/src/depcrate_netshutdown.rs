// Generated macro for Shutdown (enum)
macro_rules! Depcrate_netShutdown {
() => {
// Module: crate::net
// Provides: {"Shutdown"}
// Dependencies: {}
# [doc = " Possible values which can be passed to the [`TcpStream::shutdown`] method."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] # [stable (feature = "rust1" , since = "1.0.0")] pub enum Shutdown { # [doc = " The reading portion of the [`TcpStream`] should be shut down."] # [doc = ""] # [doc = " All currently blocked and future [reads] will return <code>[Ok]\\(0)</code>."] # [doc = ""] # [doc = " [reads]: crate::io::Read \"io::Read\""] # [stable (feature = "rust1" , since = "1.0.0")] Read , # [doc = " The writing portion of the [`TcpStream`] should be shut down."] # [doc = ""] # [doc = " All currently blocked and future [writes] will return an error."] # [doc = ""] # [doc = " [writes]: crate::io::Write \"io::Write\""] # [stable (feature = "rust1" , since = "1.0.0")] Write , # [doc = " Both the reading and the writing portions of the [`TcpStream`] should be shut down."] # [doc = ""] # [doc = " See [`Shutdown::Read`] and [`Shutdown::Write`] for more information."] # [stable (feature = "rust1" , since = "1.0.0")] Both , }
};
}
