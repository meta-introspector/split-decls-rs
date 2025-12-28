macro_rules! check_socket_for_blocking {
    () => {
        # [cfg (not (unix))] # [allow (unused_variables)] pub (crate) fn check_socket_for_blocking < S > (s : & S) -> crate :: io :: Result < () > { Ok (()) }
    };
}

check_socket_for_blocking!();