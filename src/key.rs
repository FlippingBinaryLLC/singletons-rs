use std::{
    any::TypeId,
    fmt::{Display, Formatter},
    hash::Hasher,
};

/// A `Type` represents a globally unique identifier for a type.
///
/// The properties of each `Type` come from the compiler, which are currently
/// only available for types with a static lifetime.
///
/// For a string representation of the type, there are two options. The full
/// name according to the compiler can be obtained with [`.as_str()`].
/// This may be the full path of the type, such as `"core::option::Option"`,
/// but it comes with no guarantees. A shortened version holding the last
/// segment of the type name can be obtained by calling [`.as_name()`],
/// also with no guarantees (arguably fewer guarantees). Currently,
/// `as_name()` returns the string slice within `as_str()` located between
/// the first open angle bracket (`<`) and the nearest colon (`:`) to the
/// left of it.
///
/// [`.as_str()`]: Self::as_str()
/// [`.as_name()`]: Self::as_name()
#[derive(Clone, Copy, Debug, Eq)]
pub struct Type(TypeId, &'static str);

impl Type {
    /// Creates a new `Type`
    #[must_use]
    pub fn of<T>() -> Self
    where
        T: 'static,
    {
        Type(TypeId::of::<T>(), std::any::type_name::<T>())
    }

    /// Returns a [`TypeId`] representing the type uniquely among all other
    /// types available to the compiler.
    #[must_use]
    pub fn as_id(&self) -> &TypeId {
        &self.0
    }

    /// Returns a [`TypeId`] representing the type uniquely among all other
    /// types available to the compiler.
    #[must_use]
    pub fn to_id(&self) -> TypeId {
        self.0
    }

    /// Returns a name of the type as a string, as reported by the compiler.
    ///
    /// Type names are not unique, and there may be multiple type names that
    /// all refer to the same type.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.1
    }

    // NOTE: `to_str` is not implemented as a convenience method because the
    // return value would have to be `String`, so the name `to_string` would
    // be more appropriate, but that's already implemented via the `Display`
    // implementation.

    /// Returns a short name of the type as a string.
    ///
    /// The short type name is not guaranteed to be consistent across
    /// multiple builds, or unique among available types.
    #[must_use]
    pub fn as_name(&self) -> &str {
        let to_index = self.1.find('<').unwrap_or(self.1.len());

        let from_index = self.1[..to_index].rfind(':').map_or(0, |i| i + 1);

        &self.1[from_index..to_index]
    }

    /// Returns a short name of the type as a string.
    ///
    /// The short type name is not guaranteed to be consistent across
    /// multiple builds, or unique among available types.
    #[must_use]
    pub fn to_name(&self) -> String {
        self.as_name().to_string()
    }
}

impl AsRef<str> for Type {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<TypeId> for Type {
    fn as_ref(&self) -> &TypeId {
        self.as_id()
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::hash::Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // The TypeId is guaranteed to be unique, so that's all that should
        // be hashed. The name has weaker guarantees and comes from the same
        // compiler at the same time.
        self.0.hash(state)
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        // The TypeId is guaranteed to be unique, so that's all that should
        // be hashed. The name has weaker guarantees and comes from the same
        // compiler at the same time.
        self.0 == other.0
    }
}
