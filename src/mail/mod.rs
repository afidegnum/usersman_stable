pub mod email_service;
pub mod model;
mod send_mail;

// pub use crate::auth::register_handler::*;
pub use crate::mail::email_service::*;
pub use crate::mail::model::*;
pub use crate::mail::send_mail::*;
