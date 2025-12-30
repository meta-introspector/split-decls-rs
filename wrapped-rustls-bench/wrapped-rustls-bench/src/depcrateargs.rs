// Generated macro for Args (struct)
macro_rules! DepcrateArgs {
() => {
// Module: crate
// Provides: {"Args"}
// Dependencies: {}
# [derive (Parser , Debug)] # [command (version , about = "Runs rustls benchmarks")] struct Args { # [arg (long , default_value_t = 1.0 , env = "BENCH_MULTIPLIER" , help = "Multiplies the length of every test by the given float value")] multiplier : f64 , # [arg (long , env = "BENCH_LATENCY" , help = "Writes individual handshake latency into files starting with this string.  The files are named by appending a role (client/server), a thread id, and 'latency.tsv' to the given string.")] latency_prefix : Option < String > , # [arg (long , help = "Which key type to use for server and client authentication.  The default is to run tests once for each key type.")] key_type : Option < RequestedKeyType > , # [arg (long , help = "Which provider to test")] provider : Option < Provider > , # [arg (long , default_value = "1" , help = "Number of threads to use")] threads : NonZeroUsize , # [arg (long , value_enum , default_value_t = Api :: Both , help = "Choose buffered or unbuffered API")] api : Api , # [command (subcommand)] command : Option < Command > , }
};
}
