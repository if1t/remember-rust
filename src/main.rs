use std::collections::HashMap;

fn main() {
    let source_data = [1, 1, 2, 1, 0, 3, 3, 3, 66, 0, 2, 3, 23, 2, 3];
    let mut numbers_with_count: HashMap<i32, i32> = HashMap::new();

    for  key in source_data {
        let mut count = 1;
        if numbers_with_count.contains_key(&key) {
            count = numbers_with_count[&key] + 1;
        }
        numbers_with_count.insert(key, count);
    }

    for item in numbers_with_count {
        println!("{} - {}", item.0, item.1)
    }
}
