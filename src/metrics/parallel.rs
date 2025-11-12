// Parallel batch processing for distance metrics
use pyo3::prelude::*;
use rayon::prelude::*;

/// Compute edit distances for multiple string pairs in parallel
///
/// This function processes multiple string comparisons in parallel using rayon,
/// providing significant speedup for batch operations.
///
/// # Arguments
/// * `pairs` - List of (string1, string2) tuples
/// * `substitution_cost` - Cost of substitution (default 1)
/// * `transpositions` - Whether to allow transposition edits
///
/// # Returns
/// * Vec<usize>: Edit distances for each pair
#[pyfunction]
#[pyo3(signature = (pairs, substitution_cost=1, transpositions=false))]
pub fn edit_distance_batch(
    pairs: Vec<(String, String)>,
    substitution_cost: usize,
    transpositions: bool,
) -> PyResult<Vec<usize>> {
    // Use rayon for parallel processing
    let results: Vec<usize> = pairs
        .par_iter()
        .map(|(s1, s2)| {
            super::distance::edit_distance_impl(s1, s2, substitution_cost, transpositions)
        })
        .collect();

    Ok(results)
}

/// Compute Jaro similarities for multiple string pairs in parallel
///
/// # Arguments
/// * `pairs` - List of (string1, string2) tuples
///
/// # Returns
/// * Vec<f64>: Jaro similarities for each pair
#[pyfunction]
pub fn jaro_similarity_batch(pairs: Vec<(String, String)>) -> PyResult<Vec<f64>> {
    let results: Vec<f64> = pairs
        .par_iter()
        .map(|(s1, s2)| super::distance::jaro_similarity_impl(s1, s2))
        .collect();

    Ok(results)
}

/// Compute Jaro-Winkler similarities for multiple string pairs in parallel
///
/// # Arguments
/// * `pairs` - List of (string1, string2) tuples
/// * `p` - Prefix scaling factor (default 0.1)
/// * `max_l` - Maximum prefix length (default 4)
///
/// # Returns
/// * Vec<f64>: Jaro-Winkler similarities for each pair
#[pyfunction]
#[pyo3(signature = (pairs, p=0.1, max_l=4))]
pub fn jaro_winkler_similarity_batch(
    pairs: Vec<(String, String)>,
    p: f64,
    max_l: usize,
) -> PyResult<Vec<f64>> {
    let results: Vec<f64> = pairs
        .par_iter()
        .map(|(s1, s2)| super::distance::jaro_winkler_similarity_impl(s1, s2, p, max_l))
        .collect();

    Ok(results)
}
