use super::store;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    EntityNotFound { entity: &'static str, id: i64 },

    // -- Modules
    Store(store::Error),

    // -- Externals
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),

    // --seacquery
    Sea(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error),
}

// region:    --- Froms

impl From<store::Error> for Error {
    fn from(value: store::Error) -> Self {
        Self::Store(value)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<sea_query::error::Error> for Error {
    fn from(value: sea_query::error::Error) -> Self {
        Self::Sea(value)
    }
}

// endregion: --- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
