use pyo3::{exceptions::PyKeyError, prelude::*};
use std::{collections::BTreeMap, mem::swap};

/// Iterator over the values of the map
#[pyclass]
pub(crate) struct BTreeMapValues {
    inner: Box<dyn Iterator<Item = PyObject> + Send>,
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

macro_rules! typed_btree_map {
    ($type:ty, name = $name:ident, keys_name = $keys_name:ident, items_name = $items_name:ident) => {
        /// A dictionary-like object implemented using rust's stdlib BTree, maintaining the order (by comparison) of keys
        #[pyclass]
        #[derive(Clone)]
        pub(crate) struct $name {
            inner: BTreeMap<$type, PyObject>,
        }

        /// Iterator over the keys of the map
        #[pyclass]
        pub(crate) struct $keys_name {
            inner: Box<dyn Iterator<Item = $type> + Send>,
        }

        /// Iterator over the keys and values of the map
        #[pyclass]
        pub(crate) struct $items_name {
            inner: Box<dyn Iterator<Item = ($type, PyObject)> + Send>,
        }

        #[pymethods]
        impl $name {
            #[new]
            fn new() -> Self {
                Self {
                    inner: BTreeMap::new(),
                }
            }

            fn __len__(&self) -> usize {
                self.inner.len()
            }

            fn __contains__(&self, key: $type) -> bool {
                self.inner.contains_key(&key)
            }

            fn __iter__(&self) -> $keys_name {
                self.keys()
            }

            fn __getitem__(&self, key: $type) -> PyResult<PyObject> {
                self.inner
                    .get(&key)
                    .cloned()
                    .ok_or_else(|| PyKeyError::new_err(key))
            }

            fn __setitem__(&mut self, key: $type, value: PyObject) {
                self.inner.insert(key, value);
            }

            fn __delitem__(&mut self, key: $type) {
                self.inner.remove(&key);
            }

            /// Iterator over the keys of the map. Unlike python's dictionary's `.keys()` this copies the keys into the iterator
            fn keys(&self) -> $keys_name {
                $keys_name {
                    inner: Box::new(self.inner.clone().into_keys()),
                }
            }

            /// Creates an iterator over the keys of the map. Unlike `keys()` this does not copy the keys, but it is destructive, the map will be empty after calling this
            fn keys_final(&mut self) -> $keys_name {
                let mut map = BTreeMap::new();
                swap(&mut map, &mut self.inner);

                $keys_name {
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
            fn items(&self) -> $items_name {
                $items_name {
                    inner: Box::new(self.inner.clone().into_iter()),
                }
            }

            /// Creates an iterator over the keys and values of the map. Unlike `items()` this does not copy the keys or values, but it is destructive, the map will be empty after calling this
            fn items_final(&mut self) -> $items_name {
                let mut map = BTreeMap::new();
                swap(&mut map, &mut self.inner);

                $items_name {
                    inner: Box::new(map.into_iter()),
                }
            }

            fn get(&self, key: $type) -> Option<PyObject> {
                self.inner.get(&key).cloned()
            }

            fn set(&mut self, key: $type, value: PyObject) {
                self.inner.insert(key, value);
            }

            /// Create a shallow copy of the map
            fn copy(&self) -> Self {
                self.clone()
            }
        }

        #[pymethods]
        impl $keys_name {
            fn __next__(&mut self) -> Option<$type> {
                self.inner.next()
            }

            fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
                slf
            }
        }

        #[pymethods]
        impl $items_name {
            fn __next__(&mut self) -> Option<($type, PyObject)> {
                self.inner.next()
            }

            fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
                slf
            }
        }
    };
}

typed_btree_map!(
    i64,
    name = IntBTreeMap,
    keys_name = IntBTreeMapKeys,
    items_name = IntBTreeMapItems
);

// typed_btree_map!(
//     f64,
//     name = FloatBTreeMap,
//     keys_name = FloatBTreeMapKeys,
//     items_name = FloatBTreeMapItems
// );
