use indexmap::IndexMap;
pub use indexmap::TryReserveError;

use crate::Type;

/// A map that uses types as keys and stores values of a single type `V`.
///
/// This data structure allows you to associate values with types as keys.
/// For example, you could map `u8` to a `String` description, `i8` to another
/// `String` description, etc., all within the same `SingletonMap<String>`.
///
/// # Examples
///
/// ```
/// use singletons::SingletonMap;
///
/// let mut map: SingletonMap<String> = SingletonMap::new();
/// map.insert::<u8>("An unsigned 8-bit integer".to_string());
/// map.insert::<i8>("A signed 8-bit integer".to_string());
///
/// assert_eq!(map.get::<u8>(), Some(&"An unsigned 8-bit integer".to_string()));
/// assert_eq!(map.get::<i8>(), Some(&"A signed 8-bit integer".to_string()));
/// ```
#[derive(Debug)]
pub struct SingletonMap<V>(IndexMap<Type, V>);

impl<V> SingletonMap<V> {
    /// Creates an empty `SingletonMap`.
    ///
    /// The map is initially created with a capacity of 0, so it will not
    /// allocate until an element is inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use singletons::SingletonMap;
    /// let mut map: SingletonMap<String> = SingletonMap::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        SingletonMap(IndexMap::new())
    }

    /// Creates an empty `SingletonMap` with at least the specified capacity.
    ///
    /// The map will be able to hold at least `capacity` elements without
    /// reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use singletons::SingletonMap;
    /// let mut map: SingletonMap<String> = SingletonMap::with_capacity(10);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        SingletonMap(IndexMap::with_capacity(capacity))
    }

    /// Returns the number of elements the map can hold without reallocating.
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Returns the number of elements currently in the map.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the map contains no elements.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Clears the map, removing all key-value pairs.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Reserves capacity for at least `additional` more elements.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    /// Tries to reserve capacity for at least `additional` more elements.
    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.0.try_reserve(additional)
    }

    /// Shrinks the capacity of the map as much as possible.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }

    /// Shrinks the capacity of the map as much as possible, but not less than
    /// `min_capacity`.
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.0.shrink_to(min_capacity)
    }

    /// Inserts a value into the map with the type `K` as the key.
    ///
    /// If the map did not have this type as a key, `None` is returned.
    /// If the map did have this type as a key, the value is updated and the
    /// old value is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use singletons::SingletonMap;
    ///
    /// let mut map = SingletonMap::new();
    /// assert_eq!(map.insert::<u8>("first".to_string()), None);
    /// assert_eq!(map.insert::<u8>("second".to_string()), Some("first".to_string()));
    /// ```
    pub fn insert<K: 'static>(&mut self, value: V) -> Option<V> {
        self.0.insert(Type::of::<K>(), value)
    }

    /// Returns a reference to the value corresponding to the type key `K`.
    ///
    /// # Examples
    ///
    /// ```
    /// use singletons::SingletonMap;
    ///
    /// let mut map = SingletonMap::new();
    /// map.insert::<u8>("value".to_string());
    /// assert_eq!(map.get::<u8>(), Some(&"value".to_string()));
    /// assert_eq!(map.get::<i8>(), None);
    /// ```
    #[must_use]
    pub fn get<K: 'static>(&self) -> Option<&V> {
        self.0.get(&Type::of::<K>())
    }

    /// Returns a mutable reference to the value corresponding to the type key `K`.
    ///
    /// # Examples
    ///
    /// ```
    /// use singletons::SingletonMap;
    ///
    /// let mut map = SingletonMap::new();
    /// map.insert::<u8>("value".to_string());
    /// if let Some(v) = map.get_mut::<u8>() {
    ///     v.push_str(" modified");
    /// }
    /// assert_eq!(map.get::<u8>(), Some(&"value modified".to_string()));
    /// ```
    pub fn get_mut<K: 'static>(&mut self) -> Option<&mut V> {
        self.0.get_mut(&Type::of::<K>())
    }

    /// Removes a key-value pair from the map, returning the value if the
    /// type key `K` was present.
    ///
    /// # Examples
    ///
    /// ```
    /// use singletons::SingletonMap;
    ///
    /// let mut map = SingletonMap::new();
    /// map.insert::<u8>("value".to_string());
    /// assert_eq!(map.remove::<u8>(), Some("value".to_string()));
    /// assert_eq!(map.remove::<u8>(), None);
    /// ```
    pub fn remove<K: 'static>(&mut self) -> Option<V> {
        self.0.shift_remove(&Type::of::<K>())
    }

    /// Returns true if the map contains a value for the specified type key `K`.
    ///
    /// # Examples
    ///
    /// ```
    /// use singletons::SingletonMap;
    ///
    /// let mut map = SingletonMap::new();
    /// map.insert::<u8>("value".to_string());
    /// assert!(map.contains_key::<u8>());
    /// assert!(!map.contains_key::<i8>());
    /// ```
    #[must_use]
    pub fn contains_key<K: 'static>(&self) -> bool {
        self.0.contains_key(&Type::of::<K>())
    }

    /// Returns true if any type with the given full type name is a key in the map.
    ///
    /// Type names are not guaranteed to be unique or stable across builds.
    #[must_use]
    pub fn contains_type_name(&self, type_name: &str) -> bool {
        self.0.keys().any(|t| t.as_str() == type_name)
    }

    /// Returns the [`Type`] for a given full type name, if present as a key.
    ///
    /// Type names are not guaranteed to be unique or stable across builds.
    #[must_use]
    pub fn get_type_by_name(&self, type_name: &str) -> Option<&Type> {
        self.0.keys().find(|t| t.as_str() == type_name)
    }

    /// Returns an iterator visiting all type keys in insertion order.
    #[must_use]
    pub fn keys(&self) -> Keys<'_, V> {
        Keys(self.0.keys())
    }

    /// Returns an iterator visiting all values in insertion order.
    #[must_use]
    pub fn values(&self) -> Values<'_, V> {
        Values(self.0.values())
    }

    /// Returns an iterator visiting all values mutably in insertion order.
    pub fn values_mut(&mut self) -> ValuesMut<'_, V> {
        ValuesMut(self.0.values_mut())
    }

    /// Returns an iterator visiting all key-value pairs in insertion order.
    #[must_use]
    pub fn iter(&self) -> Iter<'_, V> {
        Iter(self.0.iter())
    }

    /// Returns an iterator visiting all key-value pairs mutably in insertion order.
    pub fn iter_mut(&mut self) -> IterMut<'_, V> {
        IterMut(self.0.iter_mut())
    }

    /// Gets the given type key's corresponding entry in the map for in-place manipulation.
    ///
    /// # Examples
    ///
    /// ```
    /// use singletons::SingletonMap;
    ///
    /// let mut map = SingletonMap::new();
    /// map.entry::<u8>().or_insert("default".to_string());
    /// assert_eq!(map.get::<u8>(), Some(&"default".to_string()));
    /// ```
    pub fn entry<K: 'static>(&mut self) -> Entry<'_, V> {
        Entry {
            inner: self.0.entry(Type::of::<K>()),
        }
    }
}

impl<V: Default> SingletonMap<V> {
    /// Returns a mutable reference to the value for type key `K`, inserting
    /// a default value if the key is not present.
    ///
    /// # Examples
    ///
    /// ```
    /// use singletons::SingletonMap;
    ///
    /// let mut map: SingletonMap<String> = SingletonMap::new();
    /// let value = map.get_or_insert_default::<u8>();
    /// assert_eq!(value, &String::new());
    /// ```
    pub fn get_or_insert_default<K: 'static>(&mut self) -> &mut V {
        self.0.entry(Type::of::<K>()).or_default()
    }
}

impl<V: Default> Default for SingletonMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Clone> Clone for SingletonMap<V> {
    fn clone(&self) -> Self {
        SingletonMap(self.0.clone())
    }
}

/// An iterator over the type keys of a `SingletonMap`.
#[derive(Clone)]
pub struct Keys<'a, V>(indexmap::map::Keys<'a, Type, V>);

impl<'a, V> Iterator for Keys<'a, V> {
    type Item = &'a Type;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, V> ExactSizeIterator for Keys<'a, V> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, V> DoubleEndedIterator for Keys<'a, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

/// An iterator over the values of a `SingletonMap`.
#[derive(Clone)]
pub struct Values<'a, V>(indexmap::map::Values<'a, Type, V>);

impl<'a, V> Iterator for Values<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, V> ExactSizeIterator for Values<'a, V> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, V> DoubleEndedIterator for Values<'a, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

/// A mutable iterator over the values of a `SingletonMap`.
pub struct ValuesMut<'a, V>(indexmap::map::ValuesMut<'a, Type, V>);

impl<'a, V> Iterator for ValuesMut<'a, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, V> ExactSizeIterator for ValuesMut<'a, V> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, V> DoubleEndedIterator for ValuesMut<'a, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

/// An iterator over the key-value pairs of a `SingletonMap`.
#[derive(Clone)]
pub struct Iter<'a, V>(indexmap::map::Iter<'a, Type, V>);

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (&'a Type, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, V> ExactSizeIterator for Iter<'a, V> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, V> DoubleEndedIterator for Iter<'a, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

/// A mutable iterator over the key-value pairs of a `SingletonMap`.
pub struct IterMut<'a, V>(indexmap::map::IterMut<'a, Type, V>);

impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = (&'a Type, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, V> ExactSizeIterator for IterMut<'a, V> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, V> DoubleEndedIterator for IterMut<'a, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

/// A view into a single entry in a map, which may either be vacant or occupied.
pub struct Entry<'a, V> {
    inner: indexmap::map::Entry<'a, Type, V>,
}

impl<'a, V> Entry<'a, V> {
    /// Ensures a value is in the entry by inserting the default if empty,
    /// and returns a mutable reference to the value in the entry.
    pub fn or_insert(self, default: V) -> &'a mut V {
        self.inner.or_insert(default)
    }

    /// Ensures a value is in the entry by inserting the result of the default
    /// function if empty, and returns a mutable reference to the value in the entry.
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        self.inner.or_insert_with(default)
    }

    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the map.
    pub fn and_modify<F: FnOnce(&mut V)>(self, f: F) -> Self {
        Entry {
            inner: self.inner.and_modify(f),
        }
    }
}

impl<'a, V: Default> Entry<'a, V> {
    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    pub fn or_default(self) -> &'a mut V {
        self.inner.or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singletonmap_basic_operations() {
        let mut map = SingletonMap::new();

        assert!(map.is_empty());
        assert_eq!(map.len(), 0);

        // Insert and get
        assert_eq!(map.insert::<u8>("unsigned 8-bit".to_string()), None);
        assert_eq!(map.insert::<i8>("signed 8-bit".to_string()), None);
        assert_eq!(map.len(), 2);

        assert_eq!(map.get::<u8>(), Some(&"unsigned 8-bit".to_string()));
        assert_eq!(map.get::<i8>(), Some(&"signed 8-bit".to_string()));
        assert_eq!(map.get::<u16>(), None);

        // Update existing
        assert_eq!(
            map.insert::<u8>("new value".to_string()),
            Some("unsigned 8-bit".to_string())
        );
        assert_eq!(map.get::<u8>(), Some(&"new value".to_string()));
    }

    #[test]
    fn test_singletonmap_contains_key() {
        let mut map = SingletonMap::new();

        assert!(!map.contains_key::<u8>());
        map.insert::<u8>("value".to_string());
        assert!(map.contains_key::<u8>());
        assert!(!map.contains_key::<i8>());
    }

    #[test]
    fn test_singletonmap_remove() {
        let mut map = SingletonMap::new();

        map.insert::<u8>("value".to_string());
        assert_eq!(map.len(), 1);

        assert_eq!(map.remove::<u8>(), Some("value".to_string()));
        assert_eq!(map.len(), 0);
        assert_eq!(map.remove::<u8>(), None);
    }

    #[test]
    fn test_singletonmap_get_mut() {
        let mut map = SingletonMap::new();

        map.insert::<u8>("value".to_string());
        if let Some(v) = map.get_mut::<u8>() {
            v.push_str(" modified");
        }
        assert_eq!(map.get::<u8>(), Some(&"value modified".to_string()));
    }

    #[test]
    fn test_singletonmap_entry() {
        let mut map = SingletonMap::new();

        map.entry::<u8>().or_insert("default".to_string());
        assert_eq!(map.get::<u8>(), Some(&"default".to_string()));

        map.entry::<u8>()
            .and_modify(|v| v.push_str(" modified"))
            .or_insert("should not insert".to_string());
        assert_eq!(map.get::<u8>(), Some(&"default modified".to_string()));
    }

    #[test]
    fn test_singletonmap_get_or_insert_default() {
        let mut map: SingletonMap<String> = SingletonMap::new();

        let value = map.get_or_insert_default::<u8>();
        assert_eq!(value, &String::new());
        value.push_str("modified");
        assert_eq!(map.get::<u8>(), Some(&"modified".to_string()));
    }

    #[test]
    fn test_singletonmap_iterators() {
        let mut map = SingletonMap::new();

        map.insert::<u8>("eight".to_string());
        map.insert::<u16>("sixteen".to_string());
        map.insert::<u32>("thirty-two".to_string());

        assert_eq!(map.keys().count(), 3);
        assert_eq!(map.values().count(), 3);
        assert_eq!(map.iter().count(), 3);

        let values: Vec<&String> = map.values().collect();
        assert!(values.contains(&&"eight".to_string()));
        assert!(values.contains(&&"sixteen".to_string()));
        assert!(values.contains(&&"thirty-two".to_string()));
    }

    #[test]
    fn test_singletonmap_clear() {
        let mut map = SingletonMap::new();

        map.insert::<u8>("value".to_string());
        map.insert::<i8>("value2".to_string());
        assert_eq!(map.len(), 2);

        map.clear();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_singletonmap_clone() {
        let mut map = SingletonMap::new();
        map.insert::<u8>("value".to_string());

        let cloned = map.clone();
        assert_eq!(cloned.get::<u8>(), Some(&"value".to_string()));
    }
}
