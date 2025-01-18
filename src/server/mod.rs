pub mod admin;
pub mod config;
pub mod handlers;
pub mod middleware;
pub mod services;

pub use handlers::{login, callback, logout};
pub use middleware::{AuthenticatedUser, AuthError};