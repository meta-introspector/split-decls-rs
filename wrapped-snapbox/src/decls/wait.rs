macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! wait {
    () => {
        deps!();
        # [cfg (not (feature = "cmd"))] fn wait (mut child : std :: process :: Child , _timeout : Option < std :: time :: Duration > ,) -> std :: io :: Result < std :: process :: ExitStatus > { child . wait () }
    };
}

wait!();