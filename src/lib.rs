//! {{project-name}}

/// Returns a greeting for `name`.
///
/// ```
/// assert_eq!({{crate_name}}::greet("world"), "Hello, world!");
/// ```
#[must_use]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_by_name() {
        assert_eq!(greet("Ferris"), "Hello, Ferris!");
    }
}
