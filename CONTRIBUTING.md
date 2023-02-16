# Contributing

## Rust library

### Setup

```bash
pip install -U pre-commit
pre-commit install
```

### Run tests

```bash
cd rust
cargo test
```

## Python bindings

### Setup

```bash
pip install -U pre-commit
pre-commit install
cd python
python3 -m virtualenv venv
source venv/bin/activate
pip install -r dev-requirements.txt
```

### Run benchmarks and tests

```bash
cd python
maturin develop --release && python3 test.py
```

## Implementation notes

- According to the [Commonmark] spec, [link destinations](https://spec.commonmark.org/0.30/#link-destination) can include whitespaces if they are escaped with a backslash. This is not supported by this library as is considered a bad practice. In those cases the URLs must be encoded.

[commonmark]: https://spec.commonmark.org/0.30
