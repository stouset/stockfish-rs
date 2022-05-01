mod file;
mod rank;
mod square;

use std::fmt::{Debug, Display, Formatter, Result};
use std::marker::PhantomData;

pub use file::File;
pub use rank::Rank;
pub use square::Square;

#[derive(Clone, Copy, Debug)]
pub struct TryFromPrimitiveError<E: Debug, P: Debug + Display> {
    value: P,
    _enum: PhantomData<E>,
}

impl<E: Debug, P: Debug + Display> TryFromPrimitiveError<E, P> {
    const fn new(value: P) -> Self {
        Self { value, _enum: PhantomData }
    }
}

impl<E: Debug, P: Debug + Display> std::error::Error for TryFromPrimitiveError<E, P> {
}

impl<E: Debug, P: Debug + Display> Display for TryFromPrimitiveError<E, P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "failed to interpert {} as {}",
            self.value,
            std::any::type_name::<E>(),
        )
    }
}
