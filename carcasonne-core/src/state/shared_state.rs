/// Module containing the input-related state implementation.
pub mod input_state;

/// Module containing the menu-related state implementation.
pub mod menu_state;

/// A wrapper type to represent an optional shared context.
///
/// This enum abstracts the presence or absence of a context value.
/// It is similar to `Option<Context>` but specialized for your application's
/// state management.
///
/// # Type Parameters
///
/// * `Context` - The type of the shared context. Defaults to `()`.
///
/// # Variants
///
/// * `None` - Indicates no context is available (default).
/// * `Some(Context)` - Wraps an available context value.
#[derive(Default)]
pub enum SharedContext<Context = ()> {
    #[default]
    None,
    Some(Context),
}
