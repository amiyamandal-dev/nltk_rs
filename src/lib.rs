use pyo3::prelude::*;

// Import modules
mod metrics;
mod tag;
mod cluster;
mod tokenize;

// Re-export functions for easier access
use metrics::distance::{edit_distance, jaro_similarity, jaro_winkler_similarity};
use metrics::aline::aline_align_score;
use metrics::parallel::{
    edit_distance_batch, jaro_similarity_batch, jaro_winkler_similarity_batch,
};
use tag::hmm::{hmm_backward_probability, hmm_best_path, hmm_forward_probability};
use cluster::kmeans::{
    kmeans_classify_vectorspace, kmeans_iteration, kmeans_centroid,
    euclidean_distance, cosine_distance
};
use tokenize::regexp::{
    blankline_tokenize, regexp_tokenize, regexp_tokenize_batch, wordpunct_tokenize,
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
/// ## Phase 3: K-means Clustering
/// - kmeans_classify_vectorspace() - Classify vector to nearest cluster
/// - kmeans_iteration() - Single K-means iteration
/// - kmeans_centroid() - Compute cluster centroid
/// - euclidean_distance() - Fast Euclidean distance
/// - cosine_distance() - Fast cosine distance
///
/// ## Phase 4: ALINE Phonetic Alignment
/// - aline_align_score() - Simplified ALINE phonetic alignment score
///
/// ## Parallel Batch Processing
/// - edit_distance_batch() - Compute edit distances for multiple pairs in parallel
/// - jaro_similarity_batch() - Compute Jaro similarities for multiple pairs in parallel
/// - jaro_winkler_similarity_batch() - Compute Jaro-Winkler similarities in parallel
#[pymodule]
fn nltk_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Phase 1: Distance metric functions
    m.add_function(wrap_pyfunction!(edit_distance, m)?)?;
    m.add_function(wrap_pyfunction!(jaro_similarity, m)?)?;
    m.add_function(wrap_pyfunction!(jaro_winkler_similarity, m)?)?;

    // Parallel batch processing
    m.add_function(wrap_pyfunction!(edit_distance_batch, m)?)?;
    m.add_function(wrap_pyfunction!(jaro_similarity_batch, m)?)?;
    m.add_function(wrap_pyfunction!(jaro_winkler_similarity_batch, m)?)?;

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

    // Phase 4: ALINE phonetic alignment
    m.add_function(wrap_pyfunction!(aline_align_score, m)?)?;

    // Tokenization functions
    m.add_function(wrap_pyfunction!(regexp_tokenize, m)?)?;
    m.add_function(wrap_pyfunction!(regexp_tokenize_batch, m)?)?;
    m.add_function(wrap_pyfunction!(wordpunct_tokenize, m)?)?;
    m.add_function(wrap_pyfunction!(blankline_tokenize, m)?)?;

    Ok(())
}
