# jsonmap
JsonMap for store values of multiple types value in one Map.


## Example

```
use jsonmap::JsonMap;

fn main() {
    let mut kv_map = JsonMap::new();

    let mut obj = JsonMap::new();
    obj.insert("userid", "1000230203".into());
    obj.insert("name ", "李四".into());
    obj.insert("年龄", 29.into());

    kv_map.insert("key1", 42.into());
    kv_map.insert("key2", 190.50_f64.into());
    kv_map.insert("key3", "中国".into());
    kv_map.insert(
        "key4",
        vec![1_i32.into(), "b".into(), 10.into(), 64.0.into()].into(),
    );
    kv_map.insert("userInfo", obj.into());

    // 使用 keys 方法获取所有键的集合，并打印每个键和对应的值
    println!("Keys:");
    for key in kv_map.keys() {
        if let Some(value) = kv_map.get(key) {
            println!("Key: {}, Value: {}", key, value);
        }
    }

    // 使用 values 方法获取所有值的集合的不可变引用，并打印每个值
    println!("Values:");
    for value in kv_map.values() {
        println!("Value: {}", value);
    }

    // 使用 values_mut 方法获取所有值的集合的可变引用，并修改每个值
    for value in kv_map.values_mut() {
        println!("value: {}", &value)
    }

    // 使用 iter 方法迭代访问 KvMap 中的键值对
    println!("Iterating with iter:");
    for (key, value) in kv_map.iter() {
        println!("Key: {}, Value: {}", key, value);
    }
    // 将 Map 转换为 JSON 字符串
    let json_string = serde_json::to_string(&kv_map).unwrap();
    println!("JSON String: {}", json_string);

    // 使用 into_iter 方法获取所有权，并打印每个键值对的键和值
    println!("Iterating with into_iter:");
    for (key, value) in kv_map.into_iter() {
        println!("Key: {}, Value: {}", key, value);
    }
}


```
## result

```
Keys:
Key: key3, Value: 中国
Key: key4, Value: [1, b, 10, 64]
Key: key1, Value: 42
Key: userInfo, Value: {name : 李四 , 年龄: 29 , userid: 1000230203 , }
Key: key2, Value: 190.5
Values:
Value: 中国
Value: [1, b, 10, 64]
Value: 42
Value: {name : 李四 , 年龄: 29 , userid: 1000230203 , }
Value: 190.5
value: 中国
value: [1, b, 10, 64]
value: 42
value: {name : 李四 , 年龄: 29 , userid: 1000230203 , }
value: 190.5
Iterating with iter:
Key: key3, Value: 中国
Key: key4, Value: [1, b, 10, 64]
Key: key1, Value: 42
Key: userInfo, Value: {name : 李四 , 年龄: 29 , userid: 1000230203 , }
Key: key2, Value: 190.5
JSON String: {"key3":"中国","key4":[1,"b",10,64.0],"key1":42,"userInfo":{"name ":"李四","年龄":29,"userid":"1000230203"},"key2":190.5}
Iterating with into_iter:
Key: key3, Value: 中国
Key: key4, Value: [1, b, 10, 64]
Key: key1, Value: 42
Key: userInfo, Value: {name : 李四 , 年龄: 29 , userid: 1000230203 , }
Key: key2, Value: 190.5
```

## Cargo.toml
```

[dependencies]
jsonmap =  "0.1.0"
serde_json = "1.0"

```
