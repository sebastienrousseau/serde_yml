#[cfg(test)]
mod tests {
    use serde_yml::mapping::*;
    use serde_yml::value::Value;

    /// Tests the creation of a new empty `Mapping`.
    #[test]
    fn test_mapping_new() {
        let map = Mapping::new();
        assert!(map.map.is_empty());
    }

    /// Tests the creation of a new `Mapping` with a specified capacity.
    #[test]
    fn test_mapping_with_capacity() {
        let capacity = 10;
        let map = Mapping::with_capacity(capacity);
        assert!(map.map.is_empty());
        assert!(map.map.capacity() >= capacity);
    }

    /// Tests reserving additional capacity in the `Mapping`.
    #[test]
    fn test_mapping_reserve() {
        let mut map = Mapping::new();
        let additional = 10;
        map.reserve(additional);
        assert!(map.map.capacity() >= additional);
    }

    /// Tests reserving with zero additional capacity.
    #[test]
    fn test_mapping_reserve_zero() {
        let mut map = Mapping::new();
        let additional = 0;
        map.reserve(additional);
        assert!(map.map.capacity() >= additional);
    }

    /// Tests shrinking the capacity of the `Mapping` to fit its content.
    #[test]
    fn test_mapping_shrink_to_fit() {
        let mut map = Mapping::with_capacity(100);
        map.shrink_to_fit();
        assert!(map.map.capacity() <= 100);
    }

    /// Tests inserting a key-value pair into the `Mapping`.
    #[test]
    fn test_mapping_insert() {
        let mut map = Mapping::new();
        let key = Value::String("key".to_string());
        let value = Value::String("value".to_string());
        assert!(map.insert(key.clone(), value.clone()).is_none());
        assert_eq!(map.get(&key), Some(&value));
    }

    /// Tests inserting a key-value pair to update an existing key's value.
    #[test]
    fn test_mapping_insert_update() {
        let mut map = Mapping::new();
        let key = Value::String("key".to_string());
        let value1 = Value::String("value1".to_string());
        let value2 = Value::String("value2".to_string());
        map.insert(key.clone(), value1.clone());
        assert_eq!(
            map.insert(key.clone(), value2.clone()),
            Some(value1)
        );
        assert_eq!(map.get(&key), Some(&value2));
    }

    /// Tests checking if a key exists in the `Mapping`.
    #[test]
    fn test_mapping_contains_key() {
        let mut map = Mapping::new();
        let key = Value::String("key".to_string());
        let value = Value::String("value".to_string());
        map.insert(key.clone(), value.clone());
        assert!(map.contains_key(&key));
    }

    /// Tests checking for a non-existent key in the `Mapping`.
    #[test]
    fn test_mapping_contains_key_nonexistent() {
        let map = Mapping::new();
        let key = Value::String("key".to_string());
        assert!(!map.contains_key(&key));
    }

    /// Tests retrieving the value associated with a key in the `Mapping`.
    #[test]
    fn test_mapping_get() {
        let mut map = Mapping::new();
        let key = Value::String("key".to_string());
        let value = Value::String("value".to_string());
        map.insert(key.clone(), value.clone());
        assert_eq!(map.get(&key), Some(&value));
    }

    /// Tests retrieving the value for a non-existent key in the `Mapping`.
    #[test]
    fn test_mapping_get_nonexistent() {
        let map = Mapping::new();
        let key = Value::String("key".to_string());
        assert_eq!(map.get(&key), None);
    }

    /// Tests removing a key-value pair from the `Mapping`.
    #[test]
    fn test_mapping_remove() {
        let mut map = Mapping::new();
        let key = Value::String("key".to_string());
        let value = Value::String("value".to_string());
        map.insert(key.clone(), value.clone());
        assert_eq!(map.remove(&key), Some(value));
        assert!(!map.contains_key(&key));
    }

    /// Tests getting the number of key-value pairs in the `Mapping`.
    #[test]
    fn test_mapping_len() {
        let mut map = Mapping::new();
        assert_eq!(map.len(), 0);
        let key = Value::String("key".to_string());
        let value = Value::String("value".to_string());
        map.insert(key.clone(), value.clone());
        assert_eq!(map.len(), 1);
    }

    /// Tests checking if the `Mapping` is empty.
    #[test]
    fn test_mapping_is_empty() {
        let map = Mapping::new();
        assert!(map.is_empty());
        let mut map = Mapping::new();
        let key = Value::String("key".to_string());
        let value = Value::String("value".to_string());
        map.insert(key.clone(), value.clone());
        assert!(!map.is_empty());
    }

    /// Tests clearing all key-value pairs from the `Mapping`.
    #[test]
    fn test_mapping_clear() {
        let mut map = Mapping::new();
        let key = Value::String("key".to_string());
        let value = Value::String("value".to_string());
        map.insert(key.clone(), value.clone());
        map.clear();
        assert!(map.is_empty());
    }

    /// Tests iterating over the key-value pairs in the `Mapping`.
    #[test]
    fn test_mapping_iter() {
        let mut map = Mapping::new();
        let key = Value::String("key".to_string());
        let value = Value::String("value".to_string());
        map.insert(key.clone(), value.clone());
        let mut iter = map.iter();
        let (iter_key, iter_value) = iter.next().unwrap();
        assert_eq!(iter_key, &key);
        assert_eq!(iter_value, &value);
    }

    /// Tests iterating over multiple key-value pairs in the `Mapping`.
    #[test]
    fn test_mapping_iter_multiple() {
        let mut map = Mapping::new();
        let key1 = Value::String("key1".to_string());
        let value1 = Value::String("value1".to_string());
        let key2 = Value::String("key2".to_string());
        let value2 = Value::String("value2".to_string());
        map.insert(key1.clone(), value1.clone());
        map.insert(key2.clone(), value2.clone());
        let mut iter = map.iter();
        let (iter_key1, iter_value1) = iter.next().unwrap();
        let (iter_key2, iter_value2) = iter.next().unwrap();
        assert!(
            (iter_key1 == &key1 && iter_value1 == &value1)
                || (iter_key1 == &key2 && iter_value1 == &value2)
        );
        assert!(
            (iter_key2 == &key1 && iter_value2 == &value1)
                || (iter_key2 == &key2 && iter_value2 == &value2)
        );
    }
}
