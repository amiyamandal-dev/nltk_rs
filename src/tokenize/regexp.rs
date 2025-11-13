use std::sync::Arc;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rayon::prelude::*;
use regex::{Regex, RegexBuilder};

const FLAG_IGNORECASE: u32 = 0x02;
const FLAG_LOCALE: u32 = 0x04;
const FLAG_MULTILINE: u32 = 0x08;
const FLAG_DOTALL: u32 = 0x10;
const FLAG_UNICODE: u32 = 0x20;
const FLAG_VERBOSE: u32 = 0x40;
const FLAG_ASCII: u32 = 0x100;

const DEFAULT_FLAGS: u32 = FLAG_UNICODE | FLAG_MULTILINE | FLAG_DOTALL;

fn build_regex(pattern: &str, flags: u32) -> PyResult<Regex> {
    if flags & FLAG_LOCALE != 0 {
        return Err(PyValueError::new_err(
            "re.LOCALE flag is not supported by the Rust tokenizer",
        ));
    }

    let mut builder = RegexBuilder::new(pattern);

    if flags & FLAG_ASCII != 0 {
        builder.unicode(false);
    } else if flags & FLAG_UNICODE != 0 {
        builder.unicode(true);
    }

    builder.multi_line(flags & FLAG_MULTILINE != 0);
    builder.dot_matches_new_line(flags & FLAG_DOTALL != 0);
    builder.case_insensitive(flags & FLAG_IGNORECASE != 0);
    builder.ignore_whitespace(flags & FLAG_VERBOSE != 0);

    builder
        .build()
        .map_err(|err| PyValueError::new_err(err.to_string()))
}

fn tokenize_with_regex(regex: &Regex, text: &str, gaps: bool, discard_empty: bool) -> Vec<String> {
    if gaps {
        let iter = regex.split(text);
        if discard_empty {
            iter.filter(|tok| !tok.is_empty())
                .map(|s| s.to_string())
                .collect()
        } else {
            iter.map(|s| s.to_string()).collect()
        }
    } else {
        regex
            .find_iter(text)
            .map(|m| m.as_str().to_string())
            .collect()
    }
}

#[pyfunction(signature=(text, pattern, gaps=false, discard_empty=true, flags=DEFAULT_FLAGS))]
pub fn regexp_tokenize(
    text: &str,
    pattern: &str,
    gaps: bool,
    discard_empty: bool,
    flags: u32,
) -> PyResult<Vec<String>> {
    let regex = build_regex(pattern, flags)?;
    Ok(tokenize_with_regex(&regex, text, gaps, discard_empty))
}

#[pyfunction(signature=(texts, pattern, gaps=false, discard_empty=true, flags=DEFAULT_FLAGS))]
pub fn regexp_tokenize_batch(
    texts: Vec<String>,
    pattern: &str,
    gaps: bool,
    discard_empty: bool,
    flags: u32,
) -> PyResult<Vec<Vec<String>>> {
    let regex = Arc::new(build_regex(pattern, flags)?);
    let tokens: Vec<Vec<String>> = texts
        .par_iter()
        .map(|text| tokenize_with_regex(regex.as_ref(), text, gaps, discard_empty))
        .collect();
    Ok(tokens)
}

#[pyfunction]
pub fn wordpunct_tokenize(text: &str) -> PyResult<Vec<String>> {
    let regex = build_regex(r"\\w+|[^\\w\\s]+", DEFAULT_FLAGS)?;
    Ok(tokenize_with_regex(&regex, text, false, true))
}

#[pyfunction]
pub fn blankline_tokenize(text: &str) -> PyResult<Vec<String>> {
    let regex = build_regex(r"\\s*\\n\\s*\\n\\s*", DEFAULT_FLAGS)?;
    Ok(tokenize_with_regex(&regex, text, true, true))
}
