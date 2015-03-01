//! Provides a struct to implement `Deref` for closures that return a &T.

#![deny(missing_docs)]
#![deny(warnings)]

use std::mem;
use std::ops::Deref;

/// Wrap a closure in a way that implements `Deref`.
pub struct DerefClosure<F>(pub F);

impl<'a, T, F> Deref for DerefClosure<F> where
  T: 'a,
  F: Fn() -> &'a T,
{
  type Target = T;
  fn deref<'b>(&'b self) -> &'b T {
    // `F` returns a `'a T`, so that requires that 'a outlives `self`.
    unsafe {
      mem::transmute((self.0)())
    }
  }
}

#[cfg(test)]
mod test {
  use std::ops::Deref;
  use std::rc::Rc;

  use super::DerefClosure;

  struct Foo {
    pub xs: Vec<i32>,
  }

  #[test]
  fn simple() {
    let foo = Rc::new(Foo { xs: vec!(1, 2, 3, 4) });
    let pos = foo.xs.iter().position(|&x| x == 3).unwrap();
    let x = {
      let foo = foo.clone();
      DerefClosure(move || foo.xs.get(pos).unwrap())
    };
    assert_eq!(*x.deref(), 3);
  }
}
