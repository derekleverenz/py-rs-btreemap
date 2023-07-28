use pyo3::{FromPyObject, ToPyObject};

/*
/// Wrapper around a python object reference that uses python's builtin comparison to implement rust's Ord trait.
/// This will be slower to use than any native rust types, and can panic if the python type does not implement ordering
/// or raises an exception when determining order. Not safe to use directly
#[derive(Clone, FromPyObject)]
#[pyo3(transparent)]
struct OrderedPyObject {
    inner: PyObject,
}

impl PartialOrd for OrderedPyObject {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {

    }
}

impl PartialEq for OrderedPyObject {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

*/

/// Wrapper around f64 that defines an ordering, treating NaN as greater than all non-infinite floats
/// this is consistent with how floats are sorted in python
#[derive(Copy, Clone, PartialEq, PartialOrd, FromPyObject)]
#[pyo3(transparent)]
pub(crate) struct OrderedFloat {
    inner: f64,
}

impl Eq for OrderedFloat {}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.inner, other.inner) {
            (a, b) if a.is_nan() && b.is_infinite() => std::cmp::Ordering::Less,
            (a, _) if a.is_nan() => std::cmp::Ordering::Greater,
            (_, b) if b.is_nan() => std::cmp::Ordering::Less,
            (a, b) => a.partial_cmp(&b).unwrap(),
        }
    }
}

impl ToPyObject for OrderedFloat {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.inner.to_object(py)
    }
}

// currently unable to run rust tests in a pyo3 project
/*
#[cfg(test)]
mod test {
    use crate::ordered::OrderedFloat;

    #[test]
    fn test_ordered_float() {
        assert!(OrderedFloat { inner: f64::NAN } > OrderedFloat { inner: f64::MAX });
        assert!(OrderedFloat { inner: f64::MAX } < OrderedFloat { inner: f64::NAN });
        assert!(
            OrderedFloat { inner: f64::NAN }
                < OrderedFloat {
                    inner: f64::INFINITY
                }
        );
        assert!(OrderedFloat { inner: f64::NAN } > OrderedFloat { inner: f64::NAN });
    }
}
*/
