/// This module contains the `tag` example.
pub(crate) mod tag;

/// This module contains the `emitter` example.
pub(crate) mod emitter;

/// This module contains the `util` example.
pub(crate) mod util;

/// The main function that runs all the example modules.
pub(crate) fn main() {
    // Run the example module `emitter`.
    emitter::main();

    // Run the example module `tag`.
    tag::main();

    // Run the example module `util`.
    util::main();
}
