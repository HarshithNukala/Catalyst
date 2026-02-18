#[derive(Debug, Clone, PartialEq)]
pub enum Trigger {
    Implicit,
    Explicit {keyword: String},
    Pattern(String),
}

impl Trigger {
    pub fn matches(&self, query: &str) -> Option<String> {
        match self {
            Trigger::Implicit => {
                Some(query.to_string())
            }
            Trigger::Explicit {keyword} => {
                let trimmed = query.trim();
                if trimmed.starts_with(keyword) {
                    let string = trimmed[keyword.len()..].trim_start();
                    Some(string.to_string())
                } else {
                    None
                }   
            }
            Trigger::Pattern(_pattern) => {
                None
            }
        }
    }
    pub fn is_implicit(&self) -> bool {
        matches!(self, Trigger::Implicit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implicit_trigger() {
        let trigger = Trigger::Implicit;
        assert_eq!(trigger.matches("anything"), Some("anything".to_string()));
    }

    #[test]
    fn test_explicit_trigger() {
        let trigger = Trigger::Explicit { 
            keyword: "ai".to_string() 
        };
        
        assert_eq!(
            trigger.matches("ai hello"), 
            Some("hello".to_string())
        );
        assert_eq!(
            trigger.matches("ai"), 
            Some("".to_string())
        );
        assert_eq!(trigger.matches("hello"), None);
    }
}