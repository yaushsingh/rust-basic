/*this file is prepared so that the rust is able to access
the folder http and its content through main.rs */
pub use request::Request;
pub use method::Method;

pub mod request;
pub mod method;