// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "create_default_syscall_interceptor",
decl_type: "function",
source_file: "./src/syscall_oracle.rs",
source_crate: ".",
deps: ["SyscallWrapper", "ProcessOracle", "FileSystemOracle", "NetworkOracle", "SyscallInterceptor", "TypeSafetyLevel", "OracleType"],
uses: ["TimeOracle", "Time", "MemoryOracle", "HashMap", "SyscallWrapper", "Network", "Process", "ProcessOracle", "Memory", "Strict", "FileSystemOracle", "NetworkOracle", "SyscallInterceptor", "FileSystem", "TypeSafetyLevel", "Some", "OracleType"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        SyscallWrapper!();
        ProcessOracle!();
        FileSystemOracle!();
        NetworkOracle!();
        SyscallInterceptor!();
        TypeSafetyLevel!();
        OracleType!();
    };
}

macro_rules! create_default_syscall_interceptor {
    () => {
        deps!();
        pub fn create_default_syscall_interceptor () -> SyscallInterceptor { let mut syscall_mappings = HashMap :: new () ; syscall_mappings . insert ("#[syscall=\"read\"]\nstd::fs::read" . to_string () , SyscallWrapper { original_call : "#[syscall=\"read\"]\nstd::fs::read" . to_string () , wrapper_macro : "safe_fs_read" . to_string () , oracle_type : OracleType :: FileSystem , safety_wrapper : "FileSystemOracle" . to_string () , mock_implementation : Some ("mock_file_read()" . to_string ()) , dao_policy : Some ("filesystem_read_policy" . to_string ()) , }) ; syscall_mappings . insert ("std::fs::write" . to_string () , SyscallWrapper { original_call : "std::fs::write" . to_string () , wrapper_macro : "safe_fs_write" . to_string () , oracle_type : OracleType :: FileSystem , safety_wrapper : "FileSystemOracle" . to_string () , mock_implementation : Some ("mock_file_write()" . to_string ()) , dao_policy : Some ("filesystem_write_policy" . to_string ()) , }) ; syscall_mappings . insert ("std::process::Command" . to_string () , SyscallWrapper { original_call : "std::process::Command" . to_string () , wrapper_macro : "safe_process_exec" . to_string () , oracle_type : OracleType :: Process , safety_wrapper : "ProcessOracle" . to_string () , mock_implementation : Some ("mock_process_exec()" . to_string ()) , dao_policy : Some ("process_exec_policy" . to_string ()) , }) ; syscall_mappings . insert ("std::net::TcpStream" . to_string () , SyscallWrapper { original_call : "std::net::TcpStream" . to_string () , wrapper_macro : "safe_network_connect" . to_string () , oracle_type : OracleType :: Network , safety_wrapper : "NetworkOracle" . to_string () , mock_implementation : Some ("mock_tcp_connect()" . to_string ()) , dao_policy : Some ("network_connect_policy" . to_string ()) , }) ; syscall_mappings . insert ("libc::malloc" . to_string () , SyscallWrapper { original_call : "libc::malloc" . to_string () , wrapper_macro : "safe_memory_alloc" . to_string () , oracle_type : OracleType :: Memory , safety_wrapper : "MemoryOracle" . to_string () , mock_implementation : Some ("mock_malloc()" . to_string ()) , dao_policy : Some ("memory_alloc_policy" . to_string ()) , }) ; syscall_mappings . insert ("std::time::SystemTime" . to_string () , SyscallWrapper { original_call : "std::time::SystemTime" . to_string () , wrapper_macro : "safe_time_access" . to_string () , oracle_type : OracleType :: Time , safety_wrapper : "TimeOracle" . to_string () , mock_implementation : Some ("mock_system_time()" . to_string ()) , dao_policy : Some ("time_access_policy" . to_string ()) , }) ; SyscallInterceptor { syscall_mappings , mock_mode : false , dao_governance : true , type_safety_level : TypeSafetyLevel :: Strict , } }
    };
}

create_default_syscall_interceptor!();