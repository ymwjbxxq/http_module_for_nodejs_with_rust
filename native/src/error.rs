use neon::result::Throw;
use std::fmt::{self, Debug};

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

impl From<reqwest::Error> for ApplicationError {
    fn from(e: reqwest::Error) -> ApplicationError {
        let mut url = String::new();
        if e.url().is_some() {
            url = e.url().unwrap().to_string();
        }

        if e.is_body() {
            return ApplicationError::ClientError(format!(
                "NOT FOUND: The request body is not a JSON"
            ));
        }

        if e.is_connect() {
            return ApplicationError::ClientError(format!(
                "NOT FOUND: The request {:?} is not found",
                url
            ));
        }

        if e.is_timeout() {
            return ApplicationError::ClientError(format!(
                "TIMEOUT: The request {:?} timed out while trying to connect to the remote server",
                url
            ));
        }

        if e.is_decode() {
            return ApplicationError::ClientError(format!(
                "PARSING: invalid response from server {:?}",
                url
            ));
        }

        ApplicationError::SdkError(format!("reqwest sdk error {:?}", e))
    }
}

impl From<serde_json::error::Error> for ApplicationError {
    fn from(value: serde_json::error::Error) -> ApplicationError {
        ApplicationError::ClientError(format!("Cannot convert to stirng {}", value))
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
