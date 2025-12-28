macro_rules! deps {
    () => {
        SockRef!();
        Socket!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        # [doc = " On Windows, a corresponding `From<&impl AsSocket>` implementation exists."] # [cfg (unix)] impl < 's , S > From < & 's S > for SockRef < 's > where S : AsFd , { # [doc = " The caller must ensure `S` is actually a socket."] fn from (socket : & 's S) -> Self { let fd = socket . as_fd () . as_raw_fd () ; assert ! (fd >= 0) ; SockRef { socket : ManuallyDrop :: new (unsafe { Socket :: from_raw_fd (fd) }) , _lifetime : PhantomData , } } }
    };
}

impl_44!();