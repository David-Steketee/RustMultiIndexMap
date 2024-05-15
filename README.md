# Introduction 
This was a short week long project to develop a multi-index map in rust

The idea was to implement something similar to the c++ multiindex map from Boost that I was already familiar with https://www.boost.org/doc/libs/1_78_0/libs/multi_index/doc/index.html.

I also discovered this rust crate that implement multiindex maps in the same way and decided to follow a very similar interface https://lib.rs/crates/multi_index_map.

This multiindex map is far from complete and is intended solely as a coding exercise. The rust crate above uses unsafe methods for at least mutations. My code is entirely written in safe rust (although incomplete and doesn't even support mutations for keys anyway)
My library scales as the same rate as the rust library for insertions, deletions and retrievals but is approximately 2x slower

# Build and Test
you will need to install and use the rust nightly built at the time of writing to run benchmark tests
see installation instructions here https://doc.rust-lang.org/book/appendix-07-nightly-rust.html

# Contribute
This was intended as a coding exercise and very minor demonstration of coding competency for job applications. Feel free to take this code and do what you wish. I might come back and work on this more if I get the time who knows.