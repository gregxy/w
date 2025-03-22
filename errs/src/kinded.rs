use std::fmt::{Debug, Display};
use std::str::FromStr;

pub trait KindBound: Display + FromStr + Debug + Eq + Copy + 'static {}

#[derive(Debug)]
pub struct KindedError<K: Debug> {
    kind: K,
    message: String,
}

impl<K: KindBound> KindedError<K> {
    pub fn new<S: Display>(kind: K, message: &S) -> Self {
        Self {
            kind,
            message: message.to_string(),
        }
    }

    pub fn kind(&self) -> K {
        self.kind
    }
}

impl<K: KindBound> Display for KindedError<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl<K: KindBound> std::error::Error for KindedError<K> {}

pub trait WithKind<T, K: KindBound> {
    fn with_kind(self, kind: K) -> std::result::Result<T, KindedError<K>>;
}

impl<T, K, E> WithKind<T, K> for std::result::Result<T, E>
where
    K: KindBound,
    E: Display,
{
    fn with_kind(self, kind: K) -> std::result::Result<T, KindedError<K>> {
        self.map_err(|e| KindedError::new(kind, &e))
    }
}
