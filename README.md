# Yarra - Generic, Extensible N-Dimensional Arrays in Rust

Yarra is a library that can wrap arbitrary slice-like containers in a
multidimensional array interface. It aims to be as performant as possible
while minimizing the requirements for the underlying object. The project
arose from the need to treat interfaces to external objects as arrays, and
the desire to have a fully-featured API to work with them. Yarra currently
aims to provide this for *any* structure which can provide a minimal set of
indexing operations over a container of numeric elements; it aims to make it
extremely easy to plug a container into this framework and get reasonable
performance, but support gradual optimization of the implementation for custom
containers.

## Goals

Yarra is a *very* young, very immature library which is currently missing
many features. The goals here serve as a roadmap for future development.

Some specific short-term goals are:

- [ ] Create an `ArrayContainer` trait that uses a handful of common standard
  library traits to provide default implementations of operations necessary
  for array manipulation
- [ ] Create a generic `Array` type that can wrap containers implementing an
  `ArrayContainer` trait
- [ ] Implement an API for user-friendly n-dimensional array manipulation on top
  of this `Array` type
- [ ] Provide highly performant implementations of the `ArrayContainer` trait
  for common Rust containers like `Vec<f64>`, `[i8; N]`, etc.

Some medium term goals are:

- [ ] Add automatic parallelization support using the rayon crate
- [ ] Add opt-in explicit SIM-D vectorization using the experimental `std::simd`
  module
- [ ] Add built-in, optimized support for a container using a GPU backend

Some longer term goals are:

- [ ] Extend the `Array` API to achieve (close to) feature parity with Numpy
- [ ] Add first order automatic differentiation with a similarly gradual
  implementation complexity curve as the `Array` type
- [ ] Add efficient higher-order AD by building on Betancourt 2018's geometric
  treatment of autodiff

## Principles

Development on yarra strives to provide implementations that are efficient in
memory/compute usage, but also in developer and user time. There are tradeoffs
among these things, obviously; part of the work in this library is determining
the Pareto frontier along each of these axes.

While performance (in all the ways mentioned above) is a main goal, robustness
and safety are prioritized above this. Yarra's main branch consists solely of
code that has passed CI tests for both general usage *and* easy of future
development. I also hope to add fuzz testing to minimize bugs in corner cases
I may have missed.

## Why "Yarra"?\

Ummm... spell it backwards. Thank you, I know, I'm very creative.
