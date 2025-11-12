// ALINE: Phonetic Alignment Algorithm (Simplified Implementation)
// Based on Kondrak (2002)

use pyo3::prelude::*;

// Constants from ALINE algorithm
const C_SKIP: f64 = -10.0;  // Indels
const C_SUB: f64 = 35.0;     // Substitutions
const C_EXP: f64 = 45.0;     // Expansions/compressions

/// Simplified ALINE phonetic alignment
///
/// This is a basic implementation of the ALINE algorithm for phonetic string alignment.
/// For full feature support, use the Python implementation.
///
/// # Arguments
/// * `str1` - First phonetic string
/// * `str2` - Second phonetic string
/// * `epsilon` - Threshold for near-optimal alignments (0.0-1.0)
///
/// # Returns
/// * Alignment score (higher is better)
#[pyfunction]
#[pyo3(signature = (str1, str2, epsilon=0.0))]
pub fn aline_align_score(
    str1: &str,
    str2: &str,
    epsilon: f64,
) -> PyResult<f64> {
    if !(0.0..=1.0).contains(&epsilon) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Epsilon must be between 0.0 and 1.0"
        ));
    }

    let m = str1.chars().count();
    let n = str2.chars().count();

    let s1: Vec<char> = str1.chars().collect();
    let s2: Vec<char> = str2.chars().collect();

    // Dynamic programming matrix
    let mut score_matrix = vec![vec![0.0; n + 1]; m + 1];

    // Fill matrix with Smith-Waterman-like local alignment
    for i in 1..=m {
        for j in 1..=n {
            // Skip operations
            let skip1 = score_matrix[i - 1][j] + C_SKIP;
            let skip2 = score_matrix[i][j - 1] + C_SKIP;

            // Substitution (simplified - uses simple character matching)
            let sub_score = if s1[i - 1] == s2[j - 1] {
                C_SUB  // Match
            } else {
                C_SUB / 2.0  // Mismatch
            };
            let subst = score_matrix[i - 1][j - 1] + sub_score;

            // Expansion/compression (if applicable)
            let mut exp1 = f64::NEG_INFINITY;
            let mut exp2 = f64::NEG_INFINITY;

            if i > 1 {
                exp1 = score_matrix[i - 2][j - 1] + C_EXP / 2.0;
            }
            if j > 1 {
                exp2 = score_matrix[i - 1][j - 2] + C_EXP / 2.0;
            }

            // Take maximum, but allow 0 (local alignment)
            score_matrix[i][j] = skip1.max(skip2).max(subst).max(exp1).max(exp2).max(0.0);
        }
    }

    // Find maximum score in matrix
    let max_score = score_matrix
        .iter()
        .flat_map(|row| row.iter())
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    Ok(max_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aline_basic() {
        let score = aline_align_score("test", "test", 0.0).unwrap();
        assert!(score > 0.0);  // Identical strings should have positive score

        let score2 = aline_align_score("test", "text", 0.0).unwrap();
        assert!(score2 > 0.0);  // Similar strings should have positive score
    }
}
