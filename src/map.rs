use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::hash::Hash;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::iter::Iterator;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonMap<K>
where
    K: Eq + Clone + Hash,
{
    #[serde(flatten)]
    inner: HashMap<K, JsonV<K>>,
}

impl<K> JsonMap<K>
where
    K: Eq + Clone + Hash,
{
    // 创建一个新的 Map 实例
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    // 插入键值对到 Map 中
    pub fn insert(&mut self, key: K, value: JsonV<K>) {
        self.inner.insert(key, value);
    }

    // 获取键对应的值的可变引用
    pub fn get_mut(&mut self, key: &K) -> Option<&mut JsonV<K>> {
        self.inner.get_mut(key)
    }

    // 获取或插入键值对，并返回对值的可变引用
    pub fn entry(&mut self, key: K) -> Entry<'_, K, JsonV<K>> {
        self.inner.entry(key)
    }

    // 获取所有键的集合
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.inner.keys()
    }

    // 获取所有值的集合的不可变引用
    pub fn values(&self) -> impl Iterator<Item = &JsonV<K>> {
        self.inner.values()
    }

    // 获取所有值的集合的可变引用
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut JsonV<K>> {
        self.inner.values_mut()
    }

    // 获取键对应的值
    pub fn get(&self, key: &K) -> Option<&JsonV<K>> {
        self.inner.get(key)
    }

    // 移除键值对
    pub fn remove(&mut self, key: &K) -> Option<JsonV<K>> {
        self.inner.remove(key)
    }

    // 检查是否包含指定的键
    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    // 获取 Map 中键值对的数量
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    // 清空 Map 中的所有键值对
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    // 实现 iter 方法，返回 Map 的不可变迭代器
    pub fn iter(&self) -> impl Iterator<Item = (&K, &JsonV<K>)> {
        self.inner.iter()
    }

    // 实现 iter_mut 方法，返回 Map 的可变迭代器
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut JsonV<K>)> {
        self.inner.iter_mut()
    }
}

impl<K> FromIterator<(K, JsonV<K>)> for JsonMap<K>
where
    K: Eq + Clone + Hash,
{
    fn from_iter<I: IntoIterator<Item = (K, JsonV<K>)>>(iter: I) -> Self {
        let inner = iter.into_iter().collect();
        Self { inner }
    }
}

// 实现 IntoIterator trait
impl<K> IntoIterator for JsonMap<K>
where
    K: Eq + Hash + Clone,
{
    type Item = (K, JsonV<K>);
    type IntoIter = std::collections::hash_map::IntoIter<K, JsonV<K>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

/// 通过枚举类型支持 HashMap 内的Vlaue值可以同时存储 浮点型，整数，字符串，数组，对象类型。
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum JsonV<K>
where
    K: Eq + Hash + Clone,
{
    Boolean(bool),
    Int64(i64),
    Int32(i32),
    Float64(f64),
    Float32(f32),
    String(String),
    Array(Vec<JsonV<K>>),
    Object(JsonMap<K>),
}

impl<K> From<bool> for JsonV<K>
where
    K: Eq + Hash + Clone,
{
    fn from(value: bool) -> Self {
        JsonV::Boolean(value)
    }
}

impl<K> From<i32> for JsonV<K>
where
    K: Eq + Hash + Clone,
{
    fn from(value: i32) -> Self {
        JsonV::Int32(value)
    }
}
impl<K> From<i64> for JsonV<K>
where
    K: Eq + Hash + Clone,
{
    fn from(value: i64) -> Self {
        JsonV::Int64(value)
    }
}

impl<K> From<f64> for JsonV<K>
where
    K: Eq + Hash + Clone,
{
    fn from(value: f64) -> Self {
        JsonV::Float64(value)
    }
}

impl<K> From<String> for JsonV<K>
where
    K: Eq + Hash + Clone,
{
    fn from(value: String) -> Self {
        JsonV::String(value)
    }
}
impl<K> From<&'static str> for JsonV<K>
where
    K: Eq + Hash + Clone,
{
    fn from(value: &'static str) -> Self {
        JsonV::String(value.to_owned())
    }
}

impl<K> From<Vec<JsonV<K>>> for JsonV<K>
where
    K: Eq + Hash + Clone,
{
    fn from(value: Vec<JsonV<K>>) -> Self {
        JsonV::Array(value)
    }
}

impl<K> From<&[JsonV<K>]> for JsonV<K>
where
    K: Eq + Hash + Clone,
{
    fn from(slice: &[JsonV<K>]) -> Self {
        JsonV::Array((&slice).to_vec())
    }
}

impl<K> From<JsonMap<K>> for JsonV<K>
where
    K: Eq + Hash + Clone,
{
    fn from(value: JsonMap<K>) -> Self {
        JsonV::Object(value)
    }
}

impl<K> Display for JsonV<K>
where
    K: Eq + std::hash::Hash + Display + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            JsonV::Boolean(val) => write!(f, "{}", val),
            JsonV::Int64(val) => write!(f, "{}", val),
            JsonV::Int32(val) => write!(f, "{}", val),
            JsonV::Float64(val) => write!(f, "{}", val),
            JsonV::Float32(val) => write!(f, "{}", val),
            JsonV::String(val) => write!(f, "{}", val),
            JsonV::Array(arr) => {
                write!(f, "[")?;
                for (idx, item) in arr.iter().enumerate() {
                    if idx > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            JsonV::Object(map) => {
                write!(f, "{{")?;
                for (k, v) in map.iter() {
                    write!(f, "{}: {} , ", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}
