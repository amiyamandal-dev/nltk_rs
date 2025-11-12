use pyo3::prelude::*;

// Import modules
mod metrics;
mod tag;
mod cluster;

// Re-export functions for easier access
use metrics::distance::{edit_distance, jaro_similarity, jaro_winkler_similarity};
use tag::hmm::{hmm_best_path, hmm_forward_probability, hmm_backward_probability};
use cluster::kmeans::{
    kmeans_classify_vectorspace, kmeans_iteration, kmeans_centroid,
    euclidean_distance, cosine_distance
};

/// A Python module implemented in Rust for NLTK optimizations.
///
/// This module provides high-performance implementations of
/// computationally intensive NLP algorithms.
///
/// ## Phase 1: String Distance Metrics (✅ Complete)
/// - edit_distance() - Levenshtein/Damerau-Levenshtein (17-62x speedup)
/// - jaro_similarity() - Jaro similarity (4-11x speedup)
/// - jaro_winkler_similarity() - Jaro-Winkler similarity (5-10x speedup)
///
/// ## Phase 2: HMM Tagging (NEW)
/// - hmm_best_path() - Viterbi algorithm for optimal state sequence
/// - hmm_forward_probability() - Forward algorithm (alpha values)
/// - hmm_backward_probability() - Backward algorithm (beta values)
///
/// ## Phase 3: K-means Clustering (NEW)
/// - kmeans_classify_vectorspace() - Classify vector to nearest cluster
/// - kmeans_iteration() - Single K-means iteration
/// - kmeans_centroid() - Compute cluster centroid
/// - euclidean_distance() - Fast Euclidean distance
/// - cosine_distance() - Fast cosine distance
#[pymodule]
fn nltk_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Phase 1: Distance metric functions
    m.add_function(wrap_pyfunction!(edit_distance, m)?)?;
    m.add_function(wrap_pyfunction!(jaro_similarity, m)?)?;
    m.add_function(wrap_pyfunction!(jaro_winkler_similarity, m)?)?;

    // Phase 2: HMM tagging functions
    m.add_function(wrap_pyfunction!(hmm_best_path, m)?)?;
    m.add_function(wrap_pyfunction!(hmm_forward_probability, m)?)?;
    m.add_function(wrap_pyfunction!(hmm_backward_probability, m)?)?;

    // Phase 3: K-means clustering functions
    m.add_function(wrap_pyfunction!(kmeans_classify_vectorspace, m)?)?;
    m.add_function(wrap_pyfunction!(kmeans_iteration, m)?)?;
    m.add_function(wrap_pyfunction!(kmeans_centroid, m)?)?;
    m.add_function(wrap_pyfunction!(euclidean_distance, m)?)?;
    m.add_function(wrap_pyfunction!(cosine_distance, m)?)?;

    Ok(())
}
