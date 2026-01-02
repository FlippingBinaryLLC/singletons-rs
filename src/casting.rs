use std::any::{Any, TypeId};

/// Methods for downcasting from an `Any`-like trait object.
///
/// This trait enables type-safe downcasting for trait objects that are subtypes
/// of [`Any`]. It is implemented for `dyn Any`, `dyn Any + Send`, and
/// `dyn Any + Send + Sync`.
///
/// Users can implement this trait for custom trait objects that are supertraits
/// of `Any` to enable use with [`SingletonSet`].
pub trait Downcast: Any {
    /// Gets the `TypeId` of the underlying concrete type.
    fn type_id(&self) -> TypeId;

    /// Attempts to downcast a reference to the concrete type `T`.
    fn downcast_ref<T: Any>(&self) -> Option<&T>;

    /// Attempts to downcast a mutable reference to the concrete type `T`.
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T>;

    /// Attempts to downcast a boxed trait object to a concrete type `T`.
    fn downcast_box<T: Any>(boxed: Box<Self>) -> Result<Box<T>, Box<Self>>;
}

/// A trait for converting a value into a boxed trait object.
///
/// This trait enables inserting concrete types into a [`SingletonSet`] that
/// uses a specific trait object bound. It is implemented for all types that
/// satisfy the trait object's bounds.
///
/// Users can implement this trait for custom trait objects to enable use
/// with [`SingletonSet`].
pub trait IntoBox<A: ?Sized + Downcast>: Any {
    /// Converts `self` into a boxed trait object.
    fn into_box(self) -> Box<A>;
}

impl Downcast for dyn Any {
    #[inline]
    fn type_id(&self) -> TypeId {
        Any::type_id(self)
    }

    #[inline]
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        <dyn Any>::downcast_ref::<T>(self)
    }

    #[inline]
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        <dyn Any>::downcast_mut::<T>(self)
    }

    #[inline]
    fn downcast_box<T: Any>(boxed: Box<Self>) -> Result<Box<T>, Box<Self>> {
        boxed.downcast::<T>()
    }
}

impl<T: Any> IntoBox<dyn Any> for T {
    #[inline]
    fn into_box(self) -> Box<dyn Any> {
        Box::new(self)
    }
}

impl Downcast for dyn Any + Send {
    #[inline]
    fn type_id(&self) -> TypeId {
        Any::type_id(self)
    }

    #[inline]
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        <dyn Any>::downcast_ref::<T>(self)
    }

    #[inline]
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        <dyn Any>::downcast_mut::<T>(self)
    }

    #[inline]
    fn downcast_box<T: Any>(boxed: Box<Self>) -> Result<Box<T>, Box<Self>> {
        boxed.downcast::<T>()
    }
}

impl<T: Any + Send> IntoBox<dyn Any + Send> for T {
    #[inline]
    fn into_box(self) -> Box<dyn Any + Send> {
        Box::new(self)
    }
}

impl Downcast for dyn Any + Send + Sync {
    #[inline]
    fn type_id(&self) -> TypeId {
        Any::type_id(self)
    }

    #[inline]
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        <dyn Any>::downcast_ref::<T>(self)
    }

    #[inline]
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        <dyn Any>::downcast_mut::<T>(self)
    }

    #[inline]
    fn downcast_box<T: Any>(boxed: Box<Self>) -> Result<Box<T>, Box<Self>> {
        boxed.downcast::<T>()
    }
}

impl<T: Any + Send + Sync> IntoBox<dyn Any + Send + Sync> for T {
    #[inline]
    fn into_box(self) -> Box<dyn Any + Send + Sync> {
        Box::new(self)
    }
}
