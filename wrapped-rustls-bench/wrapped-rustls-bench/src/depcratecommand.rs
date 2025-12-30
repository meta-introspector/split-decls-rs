// Generated macro for Command (enum)
macro_rules! DepcrateCommand {
() => {
// Module: crate
// Provides: {"Command"}
// Dependencies: {}
# [derive (Parser , Debug)] enum Command { # [command (about = "Runs bulk data benchmarks")] Bulk { # [arg (help = "Which cipher suite to use; see `list-suites` for possible values.")] cipher_suite : String , # [arg (default_value_t = 1048576 , help = "The size of each data write")] plaintext_size : u64 , # [arg (help = "Maximum TLS fragment size")] max_fragment_size : Option < usize > , } , # [command (about = "Runs full handshake speed benchmarks")] Handshake { # [arg (help = "Which cipher suite to use; see `list-suites` for possible values.")] cipher_suite : String , } , # [command (about = "Runs stateful resumed handshake speed benchmarks")] HandshakeResume { # [arg (help = "Which cipher suite to use; see `list-suites` for possible values.")] cipher_suite : String , } , # [command (about = "Runs stateless resumed handshake speed benchmarks")] HandshakeTicket { # [arg (help = "Which cipher suite to use; see `list-suites` for possible values.")] cipher_suite : String , } , # [command (about = "Runs memory benchmarks" , long_about = "This creates `count` connections in parallel (count / 2 clients connected\n\
                      to count / 2 servers), and then moves them in lock-step though the handshake.\n\
                      Once the handshake completes the client writes 1KB of data to the server.")] Memory { # [arg (help = "Which cipher suite to use; see `list-suites` for possible values.")] cipher_suite : String , # [arg (default_value_t = 1000000 , help = "How many connections to create in parallel")] count : u64 , } , # [command (about = "Lists the supported values for cipher-suite options")] ListSuites , # [command (about = "Run all tests (the default subcommand)")] AllTests , }
};
}
