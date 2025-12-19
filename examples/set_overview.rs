//! Basic SingletonSet Operations
//!
//! A `SingletonSet` is a collection that stores at most one value per type.
//! Think of it like a HashSet, but instead of hashing values, it uses the
//! *type* of each value as its unique identifier.
//!
//! This example covers the fundamental operations: inserting values,
//! checking membership, and retrieving values.

use singletons::SingletonSet;

fn main() {
    // Create an empty SingletonSet
    let mut set = SingletonSet::new();

    assert!(set.is_empty());
    assert_eq!(set.len(), 0);

    // Insert values using `insert()`. Each type gets its own "slot" in the set.
    set.insert(42u32);
    set.insert("hello");
    set.insert(std::f64::consts::PI);

    // The set now contains three elements, one of each type
    assert_eq!(set.len(), 3);

    // Check if a type is present using `contains()`
    assert!(set.contains::<u32>());
    assert!(set.contains::<&str>());
    assert!(set.contains::<f64>());
    assert!(!set.contains::<i32>()); // We never inserted an i32

    // Retrieve values using `try_get()` which returns Option<&T>
    // This is the safest way to access values
    assert_eq!(set.try_get::<u32>(), Some(&42));
    assert_eq!(set.try_get::<&str>(), Some(&"hello"));
    assert_eq!(set.try_get::<f64>(), Some(&std::f64::consts::PI));
    assert_eq!(set.try_get::<i32>(), None);

    // If you're certain a type is present, use `get()` for direct access
    // Note: This will panic if the type is not in the set!
    assert_eq!(set.get::<u32>(), &42);

    // Print what we have
    println!("Set contains {} elements:", set.len());
    println!("  u32: {}", set.get::<u32>());
    println!("  &str: {}", set.get::<&str>());
    println!("  f64: {}", set.get::<f64>());

    // You can iterate over the types stored in the set
    println!("\nTypes in set:");
    for type_key in set.types() {
        println!("  - {}", type_key.as_name());
    }
}
