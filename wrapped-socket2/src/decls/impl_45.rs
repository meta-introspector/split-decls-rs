macro_rules! deps {
    () => {
        Socket!();
        SockRef!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        # [doc = " On Unix, a corresponding `From<&impl AsFd>` implementation exists."] # [cfg (windows)] impl < 's , S > From < & 's S > for SockRef < 's > where S : AsSocket , { # [doc = " See the `From<&impl AsFd>` implementation."] fn from (socket : & 's S) -> Self { let socket = socket . as_socket () . as_raw_socket () ; assert ! (socket != windows_sys :: Win32 :: Networking :: WinSock :: INVALID_SOCKET as _) ; SockRef { socket : ManuallyDrop :: new (unsafe { Socket :: from_raw_socket (socket) }) , _lifetime : PhantomData , } } }
    };
}

impl_45!();