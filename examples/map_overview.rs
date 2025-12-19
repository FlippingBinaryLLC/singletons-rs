use singletons::SingletonMap;

fn main() {
    // Create a map that stores String descriptions for different types
    let mut type_descriptions: SingletonMap<String> = SingletonMap::new();

    // Insert descriptions for various types
    type_descriptions.insert::<u8>("An unsigned 8-bit integer (0 to 255)".to_string());
    type_descriptions.insert::<i8>("A signed 8-bit integer (-128 to 127)".to_string());
    type_descriptions.insert::<u16>("An unsigned 16-bit integer".to_string());
    type_descriptions.insert::<String>("A heap-allocated string".to_string());
    type_descriptions.insert::<Vec<u32>>("A vector of 32-bit integers".to_string());

    // Retrieve descriptions
    println!("u8: {}", type_descriptions.get::<u8>().unwrap());
    println!("i8: {}", type_descriptions.get::<i8>().unwrap());
    println!("String: {}", type_descriptions.get::<String>().unwrap());

    // Modify a description
    if let Some(desc) = type_descriptions.get_mut::<u16>() {
        desc.push_str(" (0 to 65,535)");
    }
    println!("u16: {}", type_descriptions.get::<u16>().unwrap());

    // Use entry API
    type_descriptions
        .entry::<f64>()
        .or_insert("A 64-bit floating-point number".to_string());

    // Update existing entry
    type_descriptions
        .entry::<u8>()
        .and_modify(|desc| desc.push_str(" - commonly used for bytes"))
        .or_insert("Should not be inserted".to_string());

    println!("\nAll type descriptions:");
    for (type_key, description) in type_descriptions.iter() {
        println!("  {}: {}", type_key.as_name(), description);
    }

    println!("\nTotal types documented: {}", type_descriptions.len());

    // Example with default values
    let mut counters: SingletonMap<i32> = SingletonMap::new();
    *counters.get_or_insert_default::<u8>() += 5;
    *counters.get_or_insert_default::<i8>() -= 3;

    println!("\nCounters:");
    println!("  u8 counter: {}", counters.get::<u8>().unwrap());
    println!("  i8 counter: {}", counters.get::<i8>().unwrap());
}
