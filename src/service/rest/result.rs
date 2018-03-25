use diesel;
use diesel::result::{Error as DieselError, QueryResult};
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket_contrib::Json;
use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use serde_json;
use std::convert::From;
use std::io::Cursor;

/// An error returned by the API that is serialized and sent back to the UI.
///
/// NOTE: The `http_status` field is not serialized into the JSON.
///       It is used to set the HTTP response code.
#[derive(Debug)]
pub struct ApiError {

    /// An HTTP status code for the response
    http_status: u16,

    /// A detailed message describing the error
    message: String,
}

/// A custom serializer for [ApiError] that adds a "status": "error" field and
/// removes the `http_status` field.
impl Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("status", "error")?;
        map.serialize_entry("message", &self.message)?;
        map.end()
    }
}

/// Convert from database errors into ApiErrors
impl From<DieselError> for ApiError {
    fn from(err: DieselError) -> Self {
        ApiError {
            http_status: match err {
                DieselError::NotFound => 404,
                _ => 500,
            },
            message: match err {
                DieselError::NotFound => "Entity not found in database".to_string(),
                _ => err.to_string(),
            },
        }
    }
}

/// Allows responding with an [ApiError].
/// NOTE: This sets the response status to match the `http_status` field.
impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
        Response::build()
            .status(Status::from_code(self.http_status)
                .expect(&format!("Unknown HTTP status code {}", self.http_status))
            )
            .sized_body(Cursor::new(serde_json::to_string(&self)
                .expect(&format!("Could not serialize {:?} as Json", self))
            ))
            .ok()
    }
}

/// All public REST service functions should return some flavor of this [ApiResult] type.
pub type ApiResult<T> = Result<T, ApiError>;

/// Convert from any type `T` into an [ApiResult].
pub trait IntoApiResult<T> {
    fn into_api_result(self) -> ApiResult<T>;
}

/// Allows converting from [core::Result<T>] into an [ApiResult<Option<T>>].
/// Recovers from `http_status == 404` by returning `None`.
impl<E, T> IntoApiResult<Option<T>> for Result<T, E> where E: Into<ApiError> {
    fn into_api_result(self) -> ApiResult<Option<T>> {
        self.map(|entity| Some(entity)).or_else(|error| {
            let api_error: ApiError = error.into();
            if api_error.http_status == 404 {
                Ok(None)
            } else {
                Err(api_error)
            }
        })
    }
}

/// Allows converting from [core::Result] into an [ApiResult<T>].
/// Does not recover from any error type.
impl<E, T> IntoApiResult<T> for Result<T, E> where E: Into<ApiError> {
    fn into_api_result(self) -> ApiResult<T> {
        self.map_err(|error| error.into())
    }
}
