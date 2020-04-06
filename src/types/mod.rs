//! Leap Type Containers
//! ====
//!
//! Provides common set of types which extends the functionality of existing primitives and value types.
mod string;


pub use string::String;

/// Gets underlining type from a Leap Type Container
pub trait Type<T> {
    fn value(self) -> T;
}