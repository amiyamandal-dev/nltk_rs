# Natural Language Toolkit (NLTK)
[![PyPI](https://img.shields.io/pypi/v/nltk.svg)](https://pypi.python.org/pypi/nltk)
![CI](https://github.com/nltk/nltk/actions/workflows/ci.yml/badge.svg?branch=develop)

NLTK -- the Natural Language Toolkit -- is a suite of open source Python
modules, data sets, and tutorials supporting research and development in Natural
Language Processing. NLTK requires Python version from 3.10 up to the latest 3.14.

For documentation, please visit [nltk.org](https://www.nltk.org/).


## Rust-Accelerated Tokenizers (`nltk_rs`)

`nltk_rs` now includes a high-performance implementation of NLTK's regular
expression tokenizer suite, rewritten in Rust with [PyO3](https://pyo3.rs/).
Key capabilities include:

- Drop-in replacements for ``regexp_tokenize``, ``wordpunct_tokenize`` and
  ``blankline_tokenize`` that seamlessly fall back to the original Python code
  when the Rust extension is unavailable.
- A new ``regexp_tokenize_batch`` helper that tokenizes multiple texts in
  parallel using [Rayon](https://github.com/rayon-rs/rayon), providing
  multi-core speed-ups for large corpora.
- Full support for standard ``re`` flags such as ``MULTILINE`` and ``DOTALL``
  along with graceful fallback for locale-aware patterns.


### Building and Publishing to PyPI

The project is configured for distribution via
[PyPI](https://pypi.org/project/nltk-rs/) using ``maturin``. To build and upload
release artifacts:

```bash
pip install maturin
maturin build --release
maturin publish --username <pypi-username>
```

The package metadata (name, authorship, classifiers and README) is sourced from
``pyproject.toml`` so the generated wheels and source archives are ready for
upload without additional configuration.


## Contributing

Do you want to contribute to NLTK development? Great!
Please read [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

See also [how to contribute to NLTK](https://www.nltk.org/contribute.html).


## Donate

Have you found the toolkit helpful?  Please support NLTK development by donating
to the project via PayPal, using the link on the NLTK homepage.


## Citing

If you publish work that uses NLTK, please cite the NLTK book, as follows:

    Bird, Steven, Edward Loper and Ewan Klein (2009).
    Natural Language Processing with Python.  O'Reilly Media Inc.


## Copyright

Copyright (C) 2001-2025 NLTK Project

For license information, see [LICENSE.txt](LICENSE.txt).

[AUTHORS.md](AUTHORS.md) contains a list of everyone who has contributed to NLTK.


### Redistributing

- NLTK source code is distributed under the Apache 2.0 License.
- NLTK documentation is distributed under the Creative Commons
  Attribution-Noncommercial-No Derivative Works 3.0 United States license.
- NLTK corpora are provided under the terms given in the README file for each
  corpus; all are redistributable and available for non-commercial use.
- NLTK may be freely redistributed, subject to the provisions of these licenses.
