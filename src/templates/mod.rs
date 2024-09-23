use std::sync::Arc;
use minijinja::{value::Value};
use crate::squire;

/// Index page template that is served as HTML response for the root endpoint.
mod index;
/// Logout page template that is served as HTML response when the user decides to end the session.
mod logout;
/// Session page template that is served as HTML response when invalid/expired session tokens are received.
mod session;
/// Monitor page template that is served as HTML response for the monitor endpoint.
mod monitor;
/// Unauthorized template that is served as HTML response when the user is unauthorized.
mod unauthorized;
/// Error page template that is served as HTML response for any error message to be conveyed.
mod error;

/// Error page template that is served as HTML response for any error message to be conveyed.

/// Loads all the HTML templates' content into a Jinja Environment
///
/// # Returns
///
/// Returns the constructed `Arc` for the `Environment` object, that holds the central configuration state for templates.
/// It is also the container for all loaded templates.
pub fn environment() -> Arc<minijinja::Environment<'static>> {
    let mut env = minijinja::Environment::new();
    env.add_filter("capwords", capwords_filter);
    env.add_template_owned("index", index::get_content()).unwrap();
    env.add_template_owned("monitor", monitor::get_content()).unwrap();
    env.add_template_owned("logout", logout::get_content()).unwrap();
    env.add_template_owned("error", error::get_content()).unwrap();
    env.add_template_owned("session", session::get_content()).unwrap();
    env.add_template_owned("unauthorized", unauthorized::get_content()).unwrap();
    Arc::new(env)
}

// Custom filter function
fn capwords_filter(value: Value) -> Result<Value, minijinja::Error> {
    if let Some(val) = value.as_str() {
        if val.ends_with("_raw") {
            let parts: Vec<&str> = val.split('_').collect();
            let result = parts[..parts.len() - 1].join(" ");
            Ok(Value::from(result))
        } else {
            let result = val.replace("_", " ");
            Ok(Value::from(squire::util::capwords(&result, None)))
        }
    } else {
        panic!("capwords filter only works with strings");
    }
}
