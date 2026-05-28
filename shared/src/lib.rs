use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Greeting {
    pub message: String,
    pub version: String,
}

impl Greeting {
    pub fn demo() -> Self {
        Self {
            message: "Hello from Veloxis backend".to_owned(),
            version: "v0.1.0".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Greeting;

    #[test]
    fn greeting_has_expected_default_content() {
        let greeting = Greeting::demo();
        assert!(greeting.message.contains("Veloxis"));
        assert_eq!(greeting.version, "v0.1.0");
    }
}
