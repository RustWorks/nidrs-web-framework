use std::{any::{Any, TypeId}, collections::{HashMap, HashSet}, fmt::Debug, sync::Arc};

use crate::{AppError, AppResult};


#[derive(Default)]
pub struct Meta{
  map: HashMap<String, Box<dyn Any + Send + Sync>>,
  extend: Option<Arc<Meta>>,
}

impl Debug for Meta {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut keys = self.map.keys().collect::<HashSet<&String>>();
    if let Some(p) = &self.extend {
      let keys2 = p.map.keys().collect::<HashSet<&String>>();
      keys.extend(keys2);
    }
    f.debug_struct("Meta").field("keys", &keys).finish()
  }
}

impl Meta {
  pub fn new() -> Self {
    Meta {
        map: HashMap::new(),
        extend: None,
    }
  }

  pub fn inner(&self) -> &HashMap<String, Box<dyn Any + Send + Sync>> {
    &self.map
  }

  #[inline]
  pub fn with_capacity(capacity: usize) -> Self {
      Meta {
          map: HashMap::with_capacity(capacity),
          extend: None,
      }
  }

  pub fn capacity(&self) -> usize {
      self.map.capacity()
  }

  pub fn set<K: Into<String>, V: Any + Send + Sync>(&mut self, key: K, value: V)->&mut Self {
    self.map.insert(key.into(), Box::new(value));
    self
  }

  pub fn get<V: Any + Send + Sync>(&self, key: &str) -> AppResult<&V> {
    let t = self.map.get(key).and_then(|v| v.downcast_ref::<V>());
    let r = match t {
      Some(v) => Ok(v),
      None => Err(AppError::MetaNotFoundError(key.to_string())),
    };
    match r {
      Ok(v) => Ok(v),
      Err(_) => {
        match &self.extend {
          Some(p) => p.get::<V>(key),
          None => Err(AppError::MetaNotFoundError(key.to_string())),
        }
      }
    }
  }

  pub fn get_mut<V: Any + Send + Sync>(&mut self, key: &str) -> AppResult<&mut V> {
    let t = self.map.get_mut(key).and_then(|v| v.downcast_mut::<V>());
    match t {
      Some(v) => Ok(v),
      None => Err(AppError::MetaNotFoundError(key.to_string())),
    }
  }

  pub fn set_value<V: Any + Send + Sync>(&mut self, value: V)->&mut Self{
    self.set(type_key::<V>(), Box::new(value))
  }

  pub fn get_value<V: Any + Send + Sync>(&self) -> AppResult<&V> {
    self.get(&type_key::<V>())
  }

  pub fn get_mut_value<V: Any + Send + Sync>(&mut self) -> AppResult<&mut V> {
    self.get_mut(&type_key::<V>())
  }

  pub fn contains(&self, key: &str) -> bool {
    self.map.contains_key(key)
  }

  pub fn contains_value<V: Any + Send + Sync>(&self) -> bool {
    self.contains(&type_key::<V>())
  }

  pub fn remove(&mut self, key: &str) -> AppResult<Box<dyn Any + Send + Sync>> {
    let t = self.map.remove(key);
    match t {
      Some(v) => Ok(v),
      None => Err(AppError::MetaNotFoundError(key.to_string())),
    }
  }

  pub fn remove_value<V: Any + Send + Sync>(&mut self) -> AppResult<Box<dyn Any + Send + Sync>> {
    self.remove(&type_key::<V>())
  }

  pub fn merge(&mut self, meta: Meta) -> &mut Self {
    for (k, v) in meta.map {
      self.map.insert(k, v);
    }
    self
  }

  pub fn extend(&mut self, meta: Arc<Meta>) -> &mut Self {
    self.extend = Some(meta);
    self
  }
}

pub fn type_key<T: 'static>() -> String {
  format!("{:?}", TypeId::of::<T>())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_meta() {
    let mut meta = Meta::new();
    meta.set("a", 1);
    meta.set("b", "2");
    meta.set("c", 3.0);
    meta.set("d", "4".to_string());
    meta.set("e", vec![1, 2, 3]);
    meta.set("f", vec!["1", "2", "3"]);
    meta.set("g", vec![1.0, 2.0, 3.0]);
    meta.set("h", vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    meta.set("i", vec![vec![1, 2], vec![3, 4]]);
    meta.set("j", vec![vec!["1", "2"], vec!["3", "4"]]);
    meta.set("k", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    meta.set("l", vec![vec!["1".to_string(), "2".to_string()], vec!["3".to_string(), "4".to_string()]]);

    assert_eq!(*meta.get::<i32>("a").unwrap(), 1);
    assert_eq!(*meta.get::<&str>("b").unwrap(), "2");
    assert_eq!(*meta.get::<f64>("c").unwrap(), 3.0);
    assert_eq!(*meta.get::<String>("d").unwrap(), "4".to_string());
    assert_eq!(*meta.get::<Vec<i32>>("e").unwrap(), vec![1, 2, 3]);
    assert_eq!(*meta.get::<Vec<&str>>("f").unwrap(), vec!["1", "2", "3"]);
    assert_eq!(*meta.get::<Vec<f64>>("g").unwrap(), vec![1.0, 2.0, 3.0]);
    assert_eq!(*meta.get::<Vec<String>>("h").unwrap(), vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    assert_eq!(*meta.get::<Vec<Vec<i32>>>("i").unwrap(), vec![vec![1, 2], vec![3, 4]]);
    assert_eq!(*meta.get::<Vec<Vec<&str>>>("j").unwrap(), vec ![vec!["1", "2"], vec!["3", "4"]]);
  }

  #[test]
  fn test_mut_meta(){
    let mut meta = Meta::new();
    meta.set("a", 1);
    meta.set("b", "2");
    meta.set("c", 3.0);
    meta.set("d", "4".to_string());
    meta.set("e", vec![1, 2, 3]);
    meta.set("f", vec!["1", "2", "3"]);
    meta.set("g", vec![1.0, 2.0, 3.0]);
    meta.set("h", vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    meta.set("i", vec![vec![1, 2], vec![3, 4]]);
    meta.set("j", vec![vec!["1", "2"], vec!["3", "4"]]);
    meta.set("k", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    meta.set("l", vec![vec!["1".to_string(), "2".to_string()], vec!["3".to_string(), "4".to_string()]]);

    *meta.get_mut::<i32>("a").unwrap() = 2;
    *meta.get_mut::<&str>("b").unwrap() = "3";
    *meta.get_mut::<f64>("c").unwrap() = 4.0;
    *meta.get_mut::<String>("d").unwrap() = "5".to_string();
    *meta.get_mut::<Vec<i32>>("e").unwrap() = vec![2, 3, 4];
    *meta.get_mut::<Vec<&str>>("f").unwrap() = vec!["2", "3", "4"];
    *meta.get_mut::<Vec<f64>>("g").unwrap() = vec![2.0, 3.0, 4.0];
    *meta.get_mut::<Vec<String>>("h").unwrap() = vec!["2".to_string(), "3".to_string(), "4".to_string()];
    *meta.get_mut::<Vec<Vec<i32>>>("i").unwrap() = vec![vec![2, 3], vec![4, 5]];
    *meta.get_mut::<Vec<Vec<&str>>>("j").unwrap() = vec![vec!["2", "3"], vec !["4", "5"]];

    assert_eq!(*meta.get::<i32>("a").unwrap(), 2);
    assert_eq!(*meta.get::<&str>("b").unwrap(), "3");
    assert_eq!(*meta.get::<f64>("c").unwrap(), 4.0);
    assert_eq!(*meta.get::<String>("d").unwrap(), "5".to_string());
    assert_eq!(*meta.get::<Vec<i32>>("e").unwrap(), vec![2, 3, 4]);
    assert_eq!(*meta.get::<Vec<&str>>("f").unwrap(), vec!["2", "3", "4"]);
    assert_eq!(*meta.get::<Vec<f64>>("g").unwrap(), vec![2.0, 3.0, 4.0]);
    assert_eq!(*meta.get::<Vec<String>>("h").unwrap(), vec!["2".to_string(), "3".to_string(), "4".to_string()]);
    assert_eq!(*meta.get::<Vec<Vec<i32>>>("i").unwrap(), vec![vec![2, 3], vec![4, 5]]);
    assert_eq!(*meta.get::<Vec<Vec<&str>>>("j").unwrap(), vec![vec!["2", "3"], vec !["4", "5"]]);
  }

  #[test]
  fn test_meta_merge(){
    let mut meta1 = Meta::new();
    meta1.set("a", 1);
    meta1.set("b", "2");
    meta1.set("c", 3.0);
    meta1.set("d", "4".to_string());
    meta1.set("e", vec![1, 2, 3]);
    meta1.set("f", vec!["1", "2", "3"]);
    meta1.set("g", vec![1.0, 2.0, 3.0]);
    meta1.set("h", vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    meta1.set("i", vec![vec![1, 2], vec![3, 4]]);
    meta1.set("j", vec![vec!["1", "2"], vec!["3", "4"]]);
    meta1.set("k", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    meta1.set("l", vec![vec!["1".to_string(), "2".to_string()], vec!["3".to_string(), "4".to_string()]]);

    let mut meta2 = Meta::new();
    meta2.set("e", vec![2, 3, 4]);
    meta2.set("f", vec!["2", "3", "4"]);
    meta2.set("g", vec![2.0, 3.0, 4.0]);
    meta2.set("h", vec!["2".to_string(), "3".to_string(), "4".to_string()]);
    meta2.set("i", vec![vec![2, 3], vec![4, 5]]);
    meta2.set("j", vec![vec!["2", "3"], vec !["4", "5"]]);

    meta1.merge(meta2);

    assert_eq!(*meta1.get::<i32>("a").unwrap(), 1);
    assert_eq!(*meta1.get::<&str>("b").unwrap(), "2");
    assert_eq!(*meta1.get::<f64>("c").unwrap(), 3.0);
    assert_eq!(*meta1.get::<String>("d").unwrap(), "4".to_string());
    assert_eq!(*meta1.get::<Vec<i32>>("e").unwrap(), vec![2, 3, 4]);
    assert_eq!(*meta1.get::<Vec<&str>>("f").unwrap(), vec!["2", "3", "4"]);
    assert_eq!(*meta1.get::<Vec<f64>>("g").unwrap(), vec![2.0, 3.0, 4.0]);
    assert_eq!(*meta1.get::<Vec<String>>("h").unwrap(), vec!["2".to_string(), "3".to_string(), "4".to_string()]);
    assert_eq!(*meta1.get::<Vec<Vec<i32>>>("i").unwrap(), vec![vec![2, 3], vec![4, 5]]);
    assert_eq!(*meta1.get::<Vec<Vec<&str>>>("j").unwrap(), vec![vec!["2", "3"], vec !["4", "5"]]);
  }

  #[test]
  fn test_meta_extend() {
    let mut meta1 = Meta::new();
    meta1.set("a", 1);
    meta1.set("b", "2");
    meta1.set("c", 3.0);
    meta1.set("d", "4".to_string());
    meta1.set("e", vec![1, 2, 3]);
    meta1.set("f", vec!["1", "2", "3"]);
    meta1.set("g", vec![1.0, 2.0, 3.0]);

    let mut meta2 = Meta::new();
    meta2.set("e", vec![2, 3, 4]);
    meta2.set("i", vec![vec![2, 3], vec![4, 5]]);
    meta2.set("j", vec![vec!["2", "3"], vec !["4", "5"]]);

    let arc_meta2 = Arc::new(meta2);

    meta1.extend(arc_meta2);

    assert_eq!(*meta1.get::<i32>("a").unwrap(), 1);
    assert_eq!(*meta1.get::<&str>("b").unwrap(), "2");
    assert_eq!(*meta1.get::<f64>("c").unwrap(), 3.0);
    assert_eq!(*meta1.get::<String>("d").unwrap(), "4".to_string());
    assert_eq!(*meta1.get::<Vec<i32>>("e").unwrap(), vec![1, 2, 3]);
    assert_eq!(*meta1.get::<Vec<&str>>("f").unwrap(), vec!["1", "2", "3"]);
    assert_eq!(*meta1.get::<Vec<f64>>("g").unwrap(), vec![1.0, 2.0, 3.0]);
    assert_eq!(*meta1.get::<Vec<Vec<i32>>>("i").unwrap(), vec![vec![2, 3], vec![4, 5]]);
    assert_eq!(*meta1.get::<Vec<Vec<&str>>>("j").unwrap(), vec![vec!["2", "3"], vec !["4", "5"]]);
  }
}