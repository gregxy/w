use std::convert::From;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct BoxedError {
    inner: Box<dyn StdError + 'static>,
}

impl fmt::Display for BoxedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<E> From<E> for BoxedError
where
    E: StdError + 'static,
{
    fn from(e: E) -> Self {
        Self { inner: Box::new(e) }
    }
}

impl BoxedError {
    pub fn new<E>(e: E) -> Self
    where
        E: StdError + 'static,
    {
        Self {
            inner: Box::from(e),
        }
    }

    pub fn get_ref(&self) -> &(dyn StdError + 'static) {
        self.inner.as_ref()
    }

    pub fn is<T: StdError + 'static>(&self) -> bool {
        self.inner.as_ref().is::<T>()
    }
}
