/// Trait for components that provide localized text
pub trait LanguageProvider {
    /// Get translated text for a key
    fn t(&self, key: &str) -> String;
    
    /// Get translated text with argument substitution
    fn t_with_args(&self, key: &str, args: &[&str]) -> String;
}