use rust_lang_book::{advanced_traits, fearless_concurrency, smart_pointers, state_pattern_blog};
use std::boxed;
use std::error;
use std::result;

type Error = boxed::Box<dyn error::Error>;
type Result<T, E = Error> = result::Result<T, E>;

/// This binary runs various exercises from the book. It's useful for testing out some of the
/// programs that we've written. It's probably better to re-write these small programs as unit
/// tests, but for now, we're treating them as small standalone programs that we can run and "see
/// what happens".

fn main() -> Result<()> {
    // state_pattern_blog::blog::Post::new();

    // advanced_traits::methods_same_names::colliding_methods::main();
    // advanced_traits::methods_same_names::associated_functions::main();
    // advanced_traits::super_traits::main();
    // advanced_traits::newtype_pattern::main();

    // smart_pointers::deref::main();
    // smart_pointers::deref::implicit_coercion::main();

    // fearless_concurrency::threads::test_multiple_messages();
    // fearless_concurrency::threads::test_multiple_producers();
    // fearless_concurrency::threads::mutexes::basic();
    // fearless_concurrency::threads::mutexes::multi_thread();
    Ok(())
}
