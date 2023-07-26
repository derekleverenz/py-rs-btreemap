mod typed_maps;

use pyo3::{pymodule, types::PyModule, PyResult, Python};
use typed_maps::{BTreeMapValues, IntBTreeMap, IntBTreeMapItems, IntBTreeMapKeys};

/// Python wrapper around rust's BTreeMap
#[pymodule]
fn rs_btree_map(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<IntBTreeMap>()?;
    m.add_class::<IntBTreeMapItems>()?;
    m.add_class::<IntBTreeMapKeys>()?;
    m.add_class::<BTreeMapValues>()?;
    Ok(())
}
