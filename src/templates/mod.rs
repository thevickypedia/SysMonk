use std::sync::Arc;

/// Index page template that is served as HTML response for the root endpoint.
mod index;
/// Logout page template that is served as HTML response when the user decides to end the session.
mod logout;
/// Session page template that is served as HTML response when invalid/expired session tokens are received.
mod session;
/// Monitor page template that is served as HTML response for the monitor endpoint.
mod monitor;
/// Monitor page template that is served as HTML response when the user is unauthorized.
mod unauthorized;

/// Error page template that is served as HTML response for any error message to be conveyed.

/// Loads all the HTML templates' content into a Jinja Environment
///
/// # Returns
///
/// Returns the constructed `Arc` for the `Environment` object, that holds the central configuration state for templates.
/// It is also the container for all loaded templates.
pub fn environment() -> Arc<minijinja::Environment<'static>> {
    let mut env = minijinja::Environment::new();
    env.add_template_owned("index", index::get_content()).unwrap();
    env.add_template_owned("monitor", monitor::get_content()).unwrap();
    env.add_template_owned("logout", logout::get_content()).unwrap();
    env.add_template_owned("session", session::get_content()).unwrap();
    env.add_template_owned("unauthorized", unauthorized::get_content()).unwrap();
    Arc::new(env)
}
