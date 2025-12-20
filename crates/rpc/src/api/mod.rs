mod handlers;
mod method;
pub mod middleware;
mod request;
mod response;
mod validator;

use request::Request;
pub use {
    handlers::{fallback::not_found, rpc::request},
    response::{ErrorBody, Response},
};
