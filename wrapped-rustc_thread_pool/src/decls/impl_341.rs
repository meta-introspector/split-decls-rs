macro_rules! deps {
    () => {
        ThreadPool!();
        Configuration!();
        ThreadPoolBuilder!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        # [allow (deprecated)] impl Configuration { # [doc = " Creates and return a valid rayon thread pool configuration, but does not initialize it."] pub fn new () -> Configuration { Configuration { builder : ThreadPoolBuilder :: new () } } # [doc = " Deprecated in favor of `ThreadPoolBuilder::build`."] pub fn build (self) -> Result < ThreadPool , Box < dyn Error + 'static > > { self . builder . build () . map_err (Box :: from) } # [doc = " Deprecated in favor of `ThreadPoolBuilder::thread_name`."] pub fn thread_name < F > (mut self , closure : F) -> Self where F : FnMut (usize) -> String + 'static , { self . builder = self . builder . thread_name (closure) ; self } # [doc = " Deprecated in favor of `ThreadPoolBuilder::num_threads`."] pub fn num_threads (mut self , num_threads : usize) -> Configuration { self . builder = self . builder . num_threads (num_threads) ; self } # [doc = " Deprecated in favor of `ThreadPoolBuilder::panic_handler`."] pub fn panic_handler < H > (mut self , panic_handler : H) -> Configuration where H : Fn (Box < dyn Any + Send >) + Send + Sync + 'static , { self . builder = self . builder . panic_handler (panic_handler) ; self } # [doc = " Deprecated in favor of `ThreadPoolBuilder::stack_size`."] pub fn stack_size (mut self , stack_size : usize) -> Self { self . builder = self . builder . stack_size (stack_size) ; self } # [doc = " Deprecated in favor of `ThreadPoolBuilder::breadth_first`."] pub fn breadth_first (mut self) -> Self { self . builder = self . builder . breadth_first () ; self } # [doc = " Deprecated in favor of `ThreadPoolBuilder::start_handler`."] pub fn start_handler < H > (mut self , start_handler : H) -> Configuration where H : Fn (usize) + Send + Sync + 'static , { self . builder = self . builder . start_handler (start_handler) ; self } # [doc = " Deprecated in favor of `ThreadPoolBuilder::exit_handler`."] pub fn exit_handler < H > (mut self , exit_handler : H) -> Configuration where H : Fn (usize) + Send + Sync + 'static , { self . builder = self . builder . exit_handler (exit_handler) ; self } # [doc = " Returns a ThreadPoolBuilder with identical parameters."] fn into_builder (self) -> ThreadPoolBuilder { self . builder } }
    };
}

impl_341!()