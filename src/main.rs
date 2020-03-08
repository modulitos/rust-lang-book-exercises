// import modules here so that they'll run in our test suite
mod different_types_blog;
mod fearless_concurrency;
mod state_pattern_blog;
use std::boxed;
use std::error;
use std::result;

type Error = boxed::Box<dyn error::Error>;
type Result<T, E = Error> = result::Result<T, E>;

fn main() -> Result<()>{
    fearless_concurrency::threads::mutexes::multi_thread();

    Ok(())
}
