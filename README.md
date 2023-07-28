# PyBTreeMap

A python wrapper around Rust's BTreeMap. Work in progress.

## Building & Development

In a virtual environment with maturin available, run `matrin develop` to build and install in the local environment. To run tests, install `pytest` in the virtual environment and run `python -m pytest`.

## Implementation Details

## Why?

I wanted to lear to use pyo3 and also had a project in which I wanted a key-ordered dictionary. Not sure yet if this is worth using over the native python dict & sort for that usecase, or just a fun learning project.

## Performance

Benchmarking TBD

## Plan

- [ ] implement a generic version
- [ ] implement a unified python interface that hides some details
- [ ] make some benchmarks
- [ ] package
