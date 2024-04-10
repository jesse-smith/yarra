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

// This project is dual-licensed under the MIT or Apache 2.0 licenses to mirror
// the licensing of the Rust programming language. You may choose either license
// to use this software. See the LICENSE-MIT and LICENSE-APACHE files for the
// full text of each license, and the below copyright notice and disclaimer for
// the Apache v2.0 license.

// Apache License, Version 2.0 Copyright Notice and Disclaimer
// 
// Copyright `2024` `Jesse Smith`
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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