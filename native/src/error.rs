use std::fmt::{self, Debug};
use neon::{result::Throw};

#[derive(Debug)]
pub enum ApplicationError {
    InitError(String),
    ClientError(String),
    InternalError(String),
    SdkError(String),
}

impl std::error::Error for ApplicationError {}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApplicationError::InitError(msg) => write!(f, "InitError: {}", msg),
            ApplicationError::ClientError(msg) => write!(f, "ClientError: {}", msg),
            ApplicationError::InternalError(msg) => write!(f, "InternalError: {}", msg),
            ApplicationError::SdkError(err) => write!(f, "SdkError: {}", err),
        }
    }
}

impl From<Throw> for ApplicationError {
    fn from(value: Throw) -> ApplicationError {
        ApplicationError::SdkError(format!("NEON sdk error {}", value))
    }
}

// impl From<neon::handle::DowncastError<neon::prelude::JsValue, neon::prelude::JsString>>
//     for ApplicationError
// {
//     fn from(
//         value: neon::handle::DowncastError<neon::prelude::JsValue, neon::prelude::JsString>,
//     ) -> ApplicationError {
//         ApplicationError::InitError(format!("Property is missing {}", value))
//     }
// }

// impl From<Result<neon::handle::Handle<'_, neon::prelude::JsObject>, neon::result::Throw>> for ApplicationError {
//   fn from(value: Result<neon::handle::Handle<'_, neon::prelude::JsObject>, neon::result::Throw>) -> ApplicationError {
//     ApplicationError::InitError(format!("Property is missing {}", value))
//   }
// }

// impl<'a, V> neon::prelude::JsResultExt<'a, V> for ApplicationError where V: neon::prelude::Value {
//     fn or_throw<'b, C: Context<'b>>(self, cx: &mut C) -> JsResult<'a, V> {
//         todo!()
//     }
// }

// impl From<ApplicationError> for Throw {
//     fn from(value: ApplicationError) -> Throw {
//     }
// }
