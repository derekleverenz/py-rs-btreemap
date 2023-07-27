mod ordered;
mod typed_maps;

use pyo3::{pymodule, types::PyModule, PyResult, Python};
use typed_maps::{
    BTreeMapValues, BytesBTreeMap, BytesBTreeMapItems, BytesBTreeMapKeys, IntBTreeMap,
    IntBTreeMapItems, IntBTreeMapKeys, StringBTreeMap, StringBTreeMapItems, StringBTreeMapKeys,
};

/// Python wrapper around rust's BTreeMap
#[pymodule]
fn rs_btree_map(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BTreeMapValues>()?;

    m.add_class::<IntBTreeMap>()?;
    m.add_class::<IntBTreeMapItems>()?;
    m.add_class::<IntBTreeMapKeys>()?;

    // m.add_class::<FloatBTreeMap>()?;
    // m.add_class::<FloatBTreeMapKeys>()?;
    // m.add_class::<FloatBTreeMapItems>()?;

    m.add_class::<StringBTreeMap>()?;
    m.add_class::<StringBTreeMapItems>()?;
    m.add_class::<StringBTreeMapKeys>()?;

    m.add_class::<BytesBTreeMap>()?;
    m.add_class::<BytesBTreeMapItems>()?;
    m.add_class::<BytesBTreeMapKeys>()?;

    Ok(())
}
