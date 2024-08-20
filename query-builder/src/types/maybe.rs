use serde::Serialize;

/// Represents a value that may or may not be present
#[derive(Default, Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Maybe<T> {
    /// When the value is present
    Some(T),
    /// When the value is absent
    #[default]
    None,
}

impl<T> Maybe<T> {
    /// Checks if the value is present
    // Consider changing this to `is_present` or `is_serialized` or `is_included` or `is_set` to avoid confusion with `Option::is_some
    pub fn is_some(&self) -> bool {
        matches!(self, Maybe::Some(_))
    }

    /// Checks if the value is absent
    // Consider changing this to `is_absent` or `is_skipped` or `is_not_set` to avoid confusion with `Option::is_none
    pub fn is_none(&self) -> bool {
        matches!(self, Maybe::None)
    }
}
