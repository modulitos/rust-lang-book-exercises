// import modules here so that they'll run in our test suite
mod different_types_blog;
mod fearless_concurrency;
mod state_pattern_blog;
mod advanced_traits;
mod smart_pointers;
use std::boxed;
use std::error;
use std::result;

type Error = boxed::Box<dyn error::Error>;
type Result<T, E = Error> = result::Result<T, E>;

fn main() -> Result<()>{
    // fearless_concurrency::threads::mutexes::multi_thread();
    // advanced_traits::methods_same_names::colliding_methods::main();
    // advanced_traits::methods_same_names::associated_functions::main();
    // advanced_traits::super_traits::main();
    // advanced_traits::newtype_pattern::main();

    // smart_pointers::deref::main();
    smart_pointers::deref::implicit_coercion::main();
    Ok(())
}
