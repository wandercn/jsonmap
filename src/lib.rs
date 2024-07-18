#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! JsonMap for store values of multiple types value in one Map.
//! Can store floating-point numbers, integers, strings, arrays, and object types simultaneously.
//! <details class="rustdoc-toggle top-doc">
//! <summary class="docblock">zh-cn</summary>
//! JsonMap包用于在一个Map中存储多种类型的值。可以同时存储 浮点型，整数，字符串，数组，对象类型。
//! </details>
//!
//! # Example
//! ```
//!use jsonmap::JsonMap;
//!
//!fn main() {
//!    let mut kv_map = JsonMap::new();
//!    let mut obj = JsonMap::new();
//!    obj.insert("userid", "1000230203".into());
//!    obj.insert("name ", "李四".into());
//!    obj.insert("年龄", 29.into());
//!
//!    kv_map.insert("key1", 42.into());
//!    kv_map.insert("key2", 190.50_f64.into());
//!    kv_map.insert("key3", "中国".into());
//!    kv_map.insert(
//!        "key4",
//!        vec![1_i32.into(), "b".into(), 10.into(), 64.0.into()].into(),
//!    );
//!    kv_map.insert("userInfo", obj.into());
//!
//!    // 使用 keys 方法获取所有键的集合，并打印每个键和对应的值
//!    println!("Keys:");
//!    for key in kv_map.keys() {
//!        if let Some(value) = kv_map.get(key) {
//!            println!("Key: {}, Value: {}", key, value);
//!        }
//!    }
//!
//!    // 使用 values 方法获取所有值的集合的不可变引用，并打印每个值
//!    println!("Values:");
//!    for value in kv_map.values() {
//!        println!("Value: {}", value);
//!    }
//!
//!    // 使用 values_mut 方法获取所有值的集合的可变引用，并修改每个值
//!    for value in kv_map.values_mut() {
//!        println!("value: {}", &value)
//!    }
//!

//!    // 使用 iter 方法迭代访问 KvMap 中的键值对
//!    println!("Iterating with iter:");
//!    for (key, value) in kv_map.iter() {
//!        println!("Key: {}, Value: {}", key, value);
//!    }
//!    // 将 Map 转换为 JSON 字符串
//!    let json_string = serde_json::to_string(&kv_map).unwrap();
//!    println!("JSON String: {}", json_string);
//!
//!    // 使用 into_iter 方法获取所有权，并打印每个键值对的键和值
//!    println!("Iterating with into_iter:");
//!    for (key, value) in kv_map.into_iter() {
//!        println!("Key: {}, Value: {}", key, value);
//!    }
//!}
//! ```
mod map;
pub use map::JsonMap;
pub use map::JsonV;
