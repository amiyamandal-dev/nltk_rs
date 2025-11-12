// Natural Language Toolkit: Distance Metrics (Rust Implementation)
// Optimized implementations of string distance algorithms

use pyo3::prelude::*;
use std::cmp::{max, min};
use std::collections::HashMap;

/// Calculate the Levenshtein edit-distance between two strings.
///
/// The edit distance is the number of characters that need to be
/// substituted, inserted, or deleted, to transform s1 into s2.
///
/// This optionally allows transposition edits (Damerau-Levenshtein distance).
///
/// # Arguments
/// * `s1` - First string
/// * `s2` - Second string
/// * `substitution_cost` - Cost of substitution (default 1)
/// * `transpositions` - Whether to allow transposition edits (default false)
///
/// # Returns
/// The edit distance as an integer
#[pyfunction]
#[pyo3(signature = (s1, s2, substitution_cost=1, transpositions=false))]
pub fn edit_distance(
    s1: &str,
    s2: &str,
    substitution_cost: usize,
    transpositions: bool,
) -> PyResult<usize> {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    // Convert strings to character vectors for efficient indexing
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    // Initialize 2D array
    let mut lev = vec![vec![0usize; len2 + 1]; len1 + 1];

    // Initialize first column and row
    for i in 0..=len1 {
        lev[i][0] = i;
    }
    for j in 0..=len2 {
        lev[0][j] = j;
    }

    if transpositions {
        // Damerau-Levenshtein with transpositions
        let mut sigma: HashMap<char, usize> = HashMap::new();
        for c in s1_chars.iter().chain(s2_chars.iter()) {
            sigma.entry(*c).or_insert(0);
        }

        let mut last_left_t = sigma.clone();

        for i in 1..=len1 {
            let mut last_right_buf = 0;

            for j in 1..=len2 {
                let c1 = s1_chars[i - 1];
                let c2 = s2_chars[j - 1];

                let last_left = *last_left_t.get(&c2).unwrap_or(&0);
                let last_right = last_right_buf;

                if c1 == c2 {
                    last_right_buf = j;
                }

                // Deletion
                let a = lev[i - 1][j] + 1;
                // Insertion
                let b = lev[i][j - 1] + 1;
                // Substitution
                let c = lev[i - 1][j - 1] + if c1 != c2 { substitution_cost } else { 0 };

                // Transposition
                let mut d = c + 1; // never picked by default
                if last_left > 0 && last_right > 0 {
                    d = lev[last_left - 1][last_right - 1] + i - last_left + j - last_right - 1;
                }

                lev[i][j] = min(min(a, b), min(c, d));
            }

            last_left_t.insert(s1_chars[i - 1], i);
        }
    } else {
        // Standard Levenshtein distance
        for i in 1..=len1 {
            for j in 1..=len2 {
                let c1 = s1_chars[i - 1];
                let c2 = s2_chars[j - 1];

                // Deletion
                let a = lev[i - 1][j] + 1;
                // Insertion
                let b = lev[i][j - 1] + 1;
                // Substitution
                let c = lev[i - 1][j - 1] + if c1 != c2 { substitution_cost } else { 0 };

                lev[i][j] = min(min(a, b), c);
            }
        }
    }

    Ok(lev[len1][len2])
}

/// Computes the Jaro similarity between 2 strings.
///
/// The Jaro distance between is the min no. of single-character transpositions
/// required to change one word into another.
///
/// Formula: jaro_sim = 0 if m = 0 else 1/3 * (m/|s_1| + m/s_2 + (m-t)/m)
///
/// where:
/// - |s_i| is the length of string s_i
/// - m is the no. of matching characters
/// - t is the half no. of possible transpositions
///
/// # Arguments
/// * `s1` - First string
/// * `s2` - Second string
///
/// # Returns
/// The Jaro similarity as a float between 0.0 and 1.0
#[pyfunction]
pub fn jaro_similarity(s1: &str, s2: &str) -> PyResult<f64> {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    let len_s1 = s1_chars.len();
    let len_s2 = s2_chars.len();

    // Handle edge cases to match Python behavior
    if len_s1 == 0 || len_s2 == 0 {
        return Ok(0.0);  // Python returns 0 for empty strings
    }

    // The upper bound of the distance for being a matched character
    let match_bound = if max(len_s1, len_s2) / 2 > 0 {
        max(len_s1, len_s2) / 2 - 1
    } else {
        0
    };

    let mut matches = 0;
    let mut flagged_1 = Vec::new();
    let mut flagged_2 = Vec::new();

    // Find matches - use HashSet for O(1) lookups like Python's list.in
    let mut flagged_2_set = std::collections::HashSet::new();

    for i in 0..len_s1 {
        let lowerbound = if i >= match_bound { i - match_bound } else { 0 };
        let upperbound = if len_s2 > 0 {
            min(i + match_bound, len_s2 - 1)
        } else {
            0
        };

        for j in lowerbound..=upperbound {
            if j < len_s2 && s1_chars[i] == s2_chars[j] && !flagged_2_set.contains(&j) {
                matches += 1;
                flagged_1.push(i);
                flagged_2.push(j);
                flagged_2_set.insert(j);
                break;
            }
        }
    }

    if matches == 0 {
        return Ok(0.0);
    }

    // Sort flagged_2 for transposition calculation
    flagged_2.sort_unstable();

    // Count transpositions
    let mut transpositions = 0;
    for (i, j) in flagged_1.iter().zip(flagged_2.iter()) {
        if s1_chars[*i] != s2_chars[*j] {
            transpositions += 1;
        }
    }

    let m = matches as f64;
    // Use integer division for transpositions like Python does: transpositions // 2
    let t_half = (transpositions / 2) as f64;
    let jaro = (1.0 / 3.0) * (
        m / len_s1 as f64 +
        m / len_s2 as f64 +
        (m - t_half) / m
    );

    Ok(jaro)
}

/// The Jaro-Winkler similarity is an extension of the Jaro similarity.
///
/// Formula: jaro_winkler_sim = jaro_sim + (l * p * (1 - jaro_sim))
///
/// where:
/// - jaro_sim is the output from the Jaro Similarity
/// - l is the length of common prefix at the start of the string
/// - p is the constant scaling factor to overweigh common prefixes
///
/// # Arguments
/// * `s1` - First string
/// * `s2` - Second string
/// * `p` - Prefix scaling factor (default 0.1, should be <= 0.25)
/// * `max_l` - Maximum prefix length to consider (default 4)
///
/// # Returns
/// The Jaro-Winkler similarity as a float between 0.0 and 1.0
#[pyfunction]
#[pyo3(signature = (s1, s2, p=0.1, max_l=4))]
pub fn jaro_winkler_similarity(
    s1: &str,
    s2: &str,
    p: f64,
    max_l: usize,
) -> PyResult<f64> {
    // Compute the Jaro similarity
    let jaro_sim = jaro_similarity(s1, s2)?;

    // Compute the prefix matches
    let mut l = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 == c2 {
            l += 1;
            if l == max_l {
                break;
            }
        } else {
            break;
        }
    }

    // Return the Jaro-Winkler similarity
    Ok(jaro_sim + (l as f64 * p * (1.0 - jaro_sim)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance_basic() {
        assert_eq!(edit_distance("rain", "shine", 1, false).unwrap(), 3);
        assert_eq!(edit_distance("saturday", "sunday", 1, false).unwrap(), 3);
        assert_eq!(edit_distance("", "", 1, false).unwrap(), 0);
        assert_eq!(edit_distance("hello", "hello", 1, false).unwrap(), 0);
    }

    #[test]
    fn test_edit_distance_transpositions() {
        assert_eq!(edit_distance("abcdef", "acbdef", 1, true).unwrap(), 1);
        assert_eq!(edit_distance("language", "lnaguage", 1, true).unwrap(), 1);
    }

    #[test]
    fn test_jaro_similarity_basic() {
        let result = jaro_similarity("", "").unwrap();
        assert_eq!(result, 1.0);

        let result = jaro_similarity("abc", "").unwrap();
        assert_eq!(result, 0.0);

        let result = jaro_similarity("martha", "marhta").unwrap();
        assert!((result - 0.944).abs() < 0.001);
    }

    #[test]
    fn test_jaro_winkler_similarity_basic() {
        let result = jaro_winkler_similarity("martha", "marhta", 0.1, 4).unwrap();
        assert!((result - 0.961).abs() < 0.001);

        let result = jaro_winkler_similarity("billy", "billy", 0.1, 4).unwrap();
        assert_eq!(result, 1.0);
    }
}
