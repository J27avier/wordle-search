# Wordle Grid Search

Rust implementation to find five five-letter words that share no letters between them.

Original implementation from Matt Parker's Stand-up Maths [video](https://www.youtube.com/watch?v=_-AfhLQfb6w).

This implementation was inspired by Fred Overflow's [video](https://www.youtube.com/watch?v=947Ewgue4DM).

The only contribution is that this is in Rust whereas his was in Java.

I think that Rust is perfect for the low-level optimization and high-level performance required for this problem. So, it was a good excuse to have as my first "real" program in Rust.

The Java implementation took about 30 sec, while mine takes about 20 sec. Run `cargo run --release`.

Here's a question for you, reader... For problems like these, should the compilation time also be counted towards the total time of the program? This is because I imagine that many of the optimizations that the compiler is doing must be very specific for this particular code.

Anyway, enjoy criticizing my n00b level Rust. It's still early in my career as a _Rustacean_.
