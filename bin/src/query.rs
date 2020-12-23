//! Queries

/// Just query
///
/// For example, REST API, `/{avatar}/name`
pub struct Query(String);

impl Query {
    // Check if the query is valid
    pub fn is_valid(&self) -> bool {
        self.0.len() > 0 && self.0.contains("/")
    }

    /// Parse query
    pub fn parse(&self) -> Vec<&str> {
        self.0.split('/').collect::<Vec<_>>()
    }
}
