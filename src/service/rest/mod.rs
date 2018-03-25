///
/// The REST module is home to models and functions that communicate with the DB on behalf
/// of the REST API.
///

/// A module containing all of the application's models.
/// NOTE: this might be converted to a directory when the number of models grows.
pub mod models;

/// Result models returned by service functions
pub mod result;

/// All user service functions
pub mod users;
