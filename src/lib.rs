use std::{collections::BTreeMap, mem::swap};

use pyo3::{exceptions::PyKeyError, prelude::*};

/// A dictionary-like object implemented using rust's stdlib BTree, maintaining the order (by comparison) of keys
#[pyclass]
#[derive(Clone)]
struct IntBTreeMap {
    inner: BTreeMap<i64, PyObject>,
}

/// Iterator over the keys of the map
#[pyclass]
struct IntBTreeMapKeys {
    inner: Box<dyn Iterator<Item = i64> + Send>,
}

/// Iterator over the values of the map
#[pyclass]
struct BTreeMapValues {
    inner: Box<dyn Iterator<Item = PyObject> + Send>,
}

/// Iterator over the keys and values of the map
#[pyclass]
struct IntBTreeMapItems {
    inner: Box<dyn Iterator<Item = (i64, PyObject)> + Send>,
}

#[pymethods]
impl BTreeMapValues {
    fn __next__(&mut self) -> Option<PyObject> {
        self.inner.next()
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
}

#[pymethods]
impl IntBTreeMapKeys {
    fn __next__(&mut self) -> Option<i64> {
        self.inner.next()
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
}

#[pymethods]
impl IntBTreeMapItems {
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
        self.keys()
    }

    fn __getitem__(&self, key: i64) -> PyResult<PyObject> {
        self.inner
            .get(&key)
            .cloned()
            .ok_or_else(|| PyKeyError::new_err(key))
    }

    fn __setitem__(&mut self, key: i64, value: PyObject) {
        self.inner.insert(key, value);
    }

    fn __delitem__(&mut self, key: i64) {
        self.inner.remove(&key);
    }

    /// Iterator over the keys of the map. Unlike python's dictionary's `.keys()` this copies the keys into the iterator
    fn keys(&self) -> IntBTreeMapKeys {
        IntBTreeMapKeys {
            inner: Box::new(self.inner.clone().into_keys()),
        }
    }

    /// Creates an iterator over the keys of the map. Unlike `keys()` this does not copy the keys, but it is destructive, the map will be empty after calling this
    fn keys_final(&mut self) -> IntBTreeMapKeys {
        let mut map = BTreeMap::new();
        swap(&mut map, &mut self.inner);

        IntBTreeMapKeys {
            inner: Box::new(map.into_keys()),
        }
    }

    /// Iterator over the values of the map. Unlike python's dictionary's `.values()` this copies the values into the iterator
    fn values(&self) -> BTreeMapValues {
        BTreeMapValues {
            inner: Box::new(self.inner.clone().into_values()),
        }
    }

    /// Creates an iterator over the keys of the map. Unlike `values()` this does not copy the values, but it is destructive, the map will be empty after calling this
    fn values_final(&mut self) -> BTreeMapValues {
        let mut map = BTreeMap::new();
        swap(&mut map, &mut self.inner);

        BTreeMapValues {
            inner: Box::new(map.into_values()),
        }
    }

    /// Iterator over the keys and values of the map. Unlike python's dictionary's `.items()` this copies the keys & values into the iterator
    fn items(&self) -> IntBTreeMapItems {
        IntBTreeMapItems {
            inner: Box::new(self.inner.clone().into_iter()),
        }
    }

    /// Creates an iterator over the keys and values of the map. Unlike `items()` this does not copy the keys or values, but it is destructive, the map will be empty after calling this
    fn items_final(&mut self) -> IntBTreeMapItems {
        let mut map = BTreeMap::new();
        swap(&mut map, &mut self.inner);

        IntBTreeMapItems {
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
