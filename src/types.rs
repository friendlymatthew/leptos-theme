use serde::{Deserialize, Serialize};

/// Defines an enumeration for UI themes.
///
/// This enum can be cloned, copied, and compared for equality.
/// It also supports serialization and deserialization for local storage.
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

// Implementation of the default value for the `Theme` enum
impl Default for Theme {
    /// provides the default theme as `Dark`
    fn default() -> Self {
        Theme::Dark
    }
}

impl Theme {
    /// Converts the `Theme` variant into a corresponding string.
    pub fn to_string(self) -> String {
        String::from(match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json;

    #[rstest]
    #[case(Theme::Light, "\"Light\"")]
    #[case(Theme::Dark, "\"Dark\"")]
    #[case(Theme::System, "\"System\"")]
    fn test_serialization(#[case] theme: Theme, #[case] expected: &'static str) {
        let serialized = serde_json::to_string(&theme).expect("Failed to serialize theme");
        assert_eq!(serialized, expected)
    }
}
