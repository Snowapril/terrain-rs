#[cfg(not(feature = "threading"))]
use std::rc;
#[cfg(feature = "threading")]
use std::sync;

#[cfg(not(feature = "threading"))]
pub type Rc<T> = rc::Rc<T>;
#[cfg(feature = "threading")]
pub type Rc<T> = sync::Rc<T>;

#[cfg(not(feature = "threading"))]
pub type Weak<T> = rc::Weak<T>;
#[cfg(feature = "threading")]
pub type Weak<T> = sync::Weak<T>;
