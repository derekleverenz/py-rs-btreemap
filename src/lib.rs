use std::{collections::BTreeMap, mem::swap};

use pyo3::prelude::*;

/// A dictionary-like object implemented using rust's stdlib BTree, maintaining the order (by comparison) of keys
#[pyclass]
#[derive(Clone)]
struct IntBTreeMap {
    inner: BTreeMap<i64, PyObject>,
}

/// Iterator over the keys of the map. Used when consuming the map
#[pyclass]
struct IntBTreeMapKeysOnce {
    inner: Box<dyn Iterator<Item = i64> + Send>,
}

/// Iterator over the keys of the map. Contains a copy of the keys, not references to the originals in the map
#[pyclass]
struct IntBTreeMapKeys {
    keys: Vec<i64>,
    cur: usize,
}

/// Iterator over the values of the map. Used when consuming the map
#[pyclass]
struct BTreeMapValuesOnce {
    inner: Box<dyn Iterator<Item = PyObject> + Send>,
}

/// Iterator over the values of the map. Contains a copy of the python reference to each value
#[pyclass]
struct BTreeMapValues {
    values: Vec<PyObject>,
    cur: usize,
}

/// Iterator over the keys and values of the map. Used when consuming the map
#[pyclass]
struct IntBTreeMapItemsOnce {
    inner: Box<dyn Iterator<Item = (i64, PyObject)> + Send>,
}

/// Iterator over the keys and values of the map. Contains a copy of the python reference to each value and a copy or reference of the key
#[pyclass]
struct IntBTreeMapItems {
    items: Vec<(i64, PyObject)>,
    cur: usize,
}

impl IntBTreeMapKeys {
    fn new(keys: Vec<i64>) -> Self {
        Self { keys, cur: 0 }
    }
}

impl BTreeMapValues {
    fn new(values: Vec<PyObject>) -> Self {
        Self { values, cur: 0 }
    }
}

impl IntBTreeMapItems {
    fn new(items: Vec<(i64, PyObject)>) -> Self {
        Self { items, cur: 0 }
    }
}

#[pymethods]
impl IntBTreeMapKeys {
    fn __next__(&mut self) -> Option<i64> {
        // TODO we can make this more memory efficient by storing it reversed and popping off the vector, and shrinking
        // might be worth exploring
        if self.cur < self.keys.len() {
            let next = self.keys[self.cur];
            self.cur += 1;
            Some(next)
        } else {
            None
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
}

#[pymethods]
impl BTreeMapValues {
    fn __next__(&mut self) -> Option<PyObject> {
        // TODO we can make this more memory efficient by storing it reversed and popping off the vector, and shrinking
        // might be worth exploring. Also we wont need to clone the reference
        if self.cur < self.values.len() {
            let next = self.values[self.cur].clone();
            self.cur += 1;
            Some(next)
        } else {
            None
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
}

#[pymethods]
impl IntBTreeMapItems {
    fn __next__(&mut self) -> Option<(i64, PyObject)> {
        // TODO we can make this more memory efficient by storing it reversed and popping off the vector, and shrinking
        // might be worth exploring. Also we wont need to clone the reference.
        if self.cur < self.items.len() {
            let next = self.items[self.cur].clone();
            self.cur += 1;
            Some(next)
        } else {
            None
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
}

#[pymethods]
impl BTreeMapValuesOnce {
    fn __next__(&mut self) -> Option<PyObject> {
        self.inner.next()
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
}

#[pymethods]
impl IntBTreeMapKeysOnce {
    fn __next__(&mut self) -> Option<i64> {
        self.inner.next()
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
}

#[pymethods]
impl IntBTreeMapItemsOnce {
    fn __next__(&mut self) -> Option<(i64, PyObject)> {
        self.inner.next()
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
}

#[pymethods]
impl IntBTreeMap {
    #[new]
    fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }

    fn __contains__(&self, key: i64) -> bool {
        self.inner.contains_key(&key)
    }

    fn __iter__(&self) -> IntBTreeMapKeys {
        // TODO the string version will have to use .cloned()
        IntBTreeMapKeys::new(self.inner.keys().copied().collect())
    }

    fn __getitem__(&self, key: i64) -> PyObject {
        if let Some(val) = self.inner.get(&key) {
            val.clone()
        } else {
            todo!("figure out how to handle the exception")
        }
    }

    fn __setitem__(&mut self, key: i64, value: PyObject) {
        self.inner.insert(key, value);
    }

    fn __delitem__(&mut self, key: i64) {
        self.inner.remove(&key);
    }

    fn keys(&self) -> IntBTreeMapKeys {
        // TODO the string version will have to use .cloned()
        IntBTreeMapKeys::new(self.inner.keys().copied().collect())
    }

    /// Creates an iterator over the keys of the map. Unlike `keys()` this does not copy the keys, but it is destructive, the map will be empty after calling this
    fn keys_final(&mut self) -> IntBTreeMapKeysOnce {
        let mut map = BTreeMap::new();
        swap(&mut map, &mut self.inner);

        IntBTreeMapKeysOnce {
            inner: Box::new(map.into_keys()),
        }
    }

    fn values(&self) -> BTreeMapValues {
        BTreeMapValues::new(self.inner.values().cloned().collect())
    }

    /// Creates an iterator over the keys of the map. Unlike `values()` this does not copy the values, but it is destructive, the map will be empty after calling this
    fn values_final(&mut self) -> BTreeMapValuesOnce {
        let mut map = BTreeMap::new();
        swap(&mut map, &mut self.inner);

        BTreeMapValuesOnce {
            inner: Box::new(map.into_values()),
        }
    }

    fn items(&self) -> IntBTreeMapItems {
        IntBTreeMapItems::new(self.inner.iter().map(|(k, v)| (*k, v.clone())).collect())
    }

    /// Creates an iterator over the keys and values of the map. Unlike `items()` this does not copy the keys or values, but it is destructive, the map will be empty after calling this
    fn items_final(&mut self) -> IntBTreeMapItemsOnce {
        let mut map = BTreeMap::new();
        swap(&mut map, &mut self.inner);

        IntBTreeMapItemsOnce {
            inner: Box::new(map.into_iter()),
        }
    }

    fn get(&self, key: i64) -> Option<PyObject> {
        self.inner.get(&key).cloned()
    }

    fn set(&mut self, key: i64, value: PyObject) {
        self.inner.insert(key, value);
    }

    /// Create a shallow copy of the map
    fn copy(&self) -> Self {
        self.clone()
    }
}

/// Python wrapper around rust's BTreeMap
#[pymodule]
fn rs_btree_map(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<IntBTreeMap>()?;
    m.add_class::<IntBTreeMapItems>()?;
    m.add_class::<IntBTreeMapKeys>()?;
    m.add_class::<BTreeMapValues>()?;
    Ok(())
}
