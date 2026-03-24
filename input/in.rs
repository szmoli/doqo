//! # Stress Test Module
//! This is a module-level doc comment. 
//! It should be associated with the "root" or the file itself.

/// A top-level constant.
pub const GLOBAL_TIMEOUT: u64 = 30; // regular line comment

/// The primary data structure for the test suite.
/// This struct has generics and lifetimes to test your string parsing.
pub struct Manager<'a, T> {
    /// A public field with docs.
    pub name: &'a str,
    /// A private field (should your extractor show this?).
    data: Vec<T>,
}

/// Traits define shared behavior.
pub trait Worker {
    /// A trait method with a default implementation.
    fn perform_task(&self) {
        println!("Task started.");
    }
}

/// An enum to test nested variants.
pub enum Signal {
    /// The start signal.
    Start,
    /// A data-carrying signal.
    Stop { 
        /// Reason for stopping.
        reason: String, 
        /// Error code.
        code: i32 
    },
}

/// A nested module to test "Scope" logic.
/// Your Symbol Table should link these symbols to this module ID.
pub mod internal_api {
    //! This inner comment belongs to `internal_api`.

    /// A struct inside a module.
    pub struct HiddenManager;

    impl HiddenManager {
        /// A method inside an impl block.
        /// This is a "Grandchild" of the `internal_api` module.
        pub fn secret_function() -> bool {
            true
        }
    }

    /// Sub-nesting: A module inside a module.
    pub mod ultra_private {
        /// A deeply nested function.
        /// FQID: `test_suite::internal_api::ultra_private::deep_dive`
        pub fn deep_dive() {}
    }
}

/// A function with complex arguments.
/// 
/// # Arguments
/// * `input` - The raw data to process.
pub async fn process_data<F>(input: F) -> Result<(), String> 
where F: Fn(u32) -> bool {
    Ok(())
}

#[cfg(test)]
mod tests {
    /// This should likely be filtered out by your "Public API" logic.
    #[test]
    fn test_internal_logic() {
        assert!(true);
    }
}