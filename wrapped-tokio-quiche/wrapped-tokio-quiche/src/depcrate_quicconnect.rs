// Generated macro for connect (function)
macro_rules! Depcrate_quicconnect {
() => {
// Module: crate::quic
// Provides: {"connect"}
// Dependencies: {}
# [doc = " Connects to an HTTP/3 server using `socket` and the default client"] # [doc = " configuration."] # [doc = ""] # [doc = " This function always uses the [`ApplicationOverQuic`] provided in"] # [doc = " [`http3::driver`](crate::http3::driver) and returns a corresponding"] # [doc = " [ClientH3Controller]. To specify a different implementation or customize the"] # [doc = " configuration, use [`connect_with_config`]."] # [doc = ""] # [doc = " # Note"] # [doc = " tokio-quiche currently only supports one client connection per socket."] # [doc = " Sharing a socket among multiple connections will lead to lost packets as"] # [doc = " both connections try to read from the shared socket."] pub async fn connect < Tx , Rx , S > (socket : S , host : Option < & str > ,) -> QuicResult < (QuicConnection , ClientH3Controller) > where Tx : DatagramSocketSend + Send + 'static , Rx : DatagramSocketRecv + Unpin + 'static , S : TryInto < Socket < Tx , Rx > > , S :: Error : std :: error :: Error + Send + Sync + 'static , { let socket : Socket < Tx , Rx > = socket . try_into () ? ; let (h3_driver , h3_controller) = ClientH3Driver :: new (Http3Settings :: default ()) ; let mut params = ConnectionParams :: default () ; params . settings . max_idle_timeout = Some (Duration :: from_secs (30)) ; Ok ((connect_with_config (socket , host , & params , h3_driver) . await ? , h3_controller ,)) }
};
}
