/// Yarra: Generic, Extensible N-Dimensional Arrays in Rust
/// 
/// Yarra is a library that can wrap arbitrary slice-like containers in a
/// multidimensional array interface. It aims to be as performant as possible
/// while minimizing the requirements for the underlying object. The project
/// arose from the need to treat interfaces to external objects as arrays, and
/// the desire to have a fully-featured API to work with them. Yarra currently
/// aims to provide this for *any* structure which can provide a minimal set of
/// indexing operations over a container of numeric elements; it aims to make it 
/// extremely easy to plug a container into this framework and get reasonable 
/// performance, but support gradual optimization of the implementation for
/// custom containers.

mod array;
mod array_container;
mod array_element;
mod array_adapter;

pub use array::Array;
pub use array_container::ArrayContainer;
pub use array_element::ArrayElement;
pub use array_adapter::{
    ArrayAdapter, AsRefSliceAdapter, BorrowSliceAdapter, DerefSliceAdapter
};