#![allow(incomplete_features)]
#![feature(
  external_doc,
  min_specialization,
  drain_filter,
  test,
  decl_macro,
  generic_associated_types
)]

//! Reactive extensions library for Rust: a library for
//! [Reactive Programming](http://reactivex.io/) using
//! [Observable](crate::observable::Observable), to make
//! it easier to compose asynchronous or callback-based code.
#![doc(include = "../README.md")]

#[cfg(test)]
extern crate float_cmp;

pub mod inner_deref;
pub mod observable;
pub mod observer;
pub mod ops;
pub mod scheduler;
pub mod shared;
pub mod subject;
pub mod subscriber;
pub mod subscription;
pub mod type_hint;

pub mod prelude {
  pub use crate::inner_deref::{InnerDeref, InnerDerefMut};
  pub use crate::observable;
  pub use crate::observable::*;
  pub use crate::observer;
  pub use crate::ops;
  pub use crate::scheduler::*;
  pub use crate::shared;
  pub use crate::subject;
  pub use crate::subject::*;
  pub use crate::subscriber::Subscriber;
  pub use crate::subscription;
  pub use crate::subscription::*;
  pub use crate::type_hint::TypeHint;
  pub use observer::Observer;
  pub use shared::*;
}
