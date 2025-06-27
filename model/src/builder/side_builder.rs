use crate::side::{Side, SideKind};

/// A builder struct for constructing instances of a `Side`.
///
/// The `SideBuilder` struct provides a builder-style API to configure and
/// construct a `Side` object by specifying its components such as `section`
/// and `kind`.
///
/// # Fields
///
/// * `section` - Represents the section of the side as an unsigned integer.
///   Defaults to `0` if not explicitly set.
/// * `kind` - Specifies the kind of side, represented by the `SideKind` enum.
///   Defaults to the default variant of `SideKind`.
///
/// # Derives
///
/// The `Default` trait is implemented for `SideBuilder`, allowing for easy
/// instantiation of a default `SideBuilder` instance. This will initialize
/// the fields with their default values.
///
/// # Example
///
/// ```
/// use model::builder::side_builder::SideBuilder;
/// let builder = SideBuilder::default();
/// ```
#[derive(Default)]
pub struct SideBuilder {
    section: usize,
    kind: SideKind,
}

impl SideBuilder {
    /// Sets the `section` field of the current object to the specified value and returns a mutable reference to the object.
    ///
    /// # Parameters
    /// - `section`: The new value to set for the `section` field. It is of the type `usize`.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing for method chaining.
    ///
    /// # Examples
    /// ```rust
    /// use model::builder::side_builder::SideBuilder;
    /// let mut obj = SideBuilder::default()
    ///     .section(5)
    ///     .another_method();
    /// assert_eq!(obj.section, 5);
    /// ```
    pub fn section(&mut self, section: usize) -> &mut Self {
        self.section = section;
        self
    }

    /// Sets the `kind` field of the current instance to the provided `SideKind` value.
    ///
    /// This method is used to modify the type or category (`kind`) of the object it is called on.
    /// It takes ownership of the provided `SideKind` value, assigns it to the internal `kind` field,
    /// and then returns a mutable reference to the current instance, enabling method chaining.
    ///
    /// # Arguments
    /// * `kind` - A value of type `SideKind` that represents the new kind or type to set.
    ///
    /// # Returns
    /// * `&mut Self` - A mutable reference to the current instance, allowing for method chaining.
    ///
    /// # Example
    /// ```
    /// use model::builder::side_builder::SideBuilder;
    /// use model::side::SideKind;
    ///
    /// let mut obj = SideBuilder::default()
    ///     .kind(SideKind::Meadow)
    ///     .other_method();
    /// ```
    pub fn kind(&mut self, kind: SideKind) -> &mut Self {
        self.kind = kind;
        self
    }

    /// Builds and returns an instance of the `Side` struct.
    ///
    /// This function constructs a `Side` object using the current values of the `section`
    /// and `kind` fields from the implementing object. It is typically used when you need
    /// to create a finalized version of the `Side` struct based on the builder’s state.
    ///
    /// # Returns
    /// A `Side` instance with properties initialized from the implementing object's fields.
    ///
    /// # Example
    /// ```
    /// use model::builder::side_builder::SideBuilder;
    /// use model::side::{ SideKind};
    /// let builder = SideBuilder::default()
    ///     .section(3)
    ///     .kind(SideKind::Meadow);
    ///
    /// let side = builder.build();
    /// println!("{:?}", side);
    /// ```
    pub fn build(&self) -> Side {
        Side {
            section: self.section,
            kind: self.kind,
        }
    }
}
