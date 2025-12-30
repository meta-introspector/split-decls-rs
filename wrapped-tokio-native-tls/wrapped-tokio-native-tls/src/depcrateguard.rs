// Generated macro for Guard (struct)
macro_rules! DepcrateGuard {
() => {
// Module: crate
// Provides: {"Guard"}
// Dependencies: {}
struct Guard < 'a , S > (& 'a mut TlsStream < S >) where AllowStd < S > : Read + Write ;
};
}
