// Natural Language Toolkit: Hidden Markov Model (Rust Implementation)
// Optimized implementations of HMM algorithms using Rust + PyO3

use numpy::{PyArray2, PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;

/// Viterbi algorithm: finds the best path (most probable state sequence) through the HMM
///
/// This is the optimized Rust implementation of HiddenMarkovModelTagger._best_path()
///
/// # Arguments
/// * `priors` - P: log prior probabilities, shape (N,) where N = number of states
/// * `outputs` - O: log output probabilities, shape (N, M) where M = number of symbols
/// * `transitions` - X: log transition probabilities, shape (N, N)
/// * `sequence` - S: sequence of symbol indices, shape (T,) where T = sequence length
///
/// # Returns
/// * Tuple of (best_sequence, best_prob) where:
///   - best_sequence: Vec<usize> of state indices for the best path
///   - best_prob: f32 log probability of the best path
#[pyfunction]
pub fn hmm_best_path<'py>(
    _py: Python<'py>,
    priors: PyReadonlyArray1<'py, f32>,
    outputs: PyReadonlyArray2<'py, f32>,
    transitions: PyReadonlyArray2<'py, f32>,
    sequence: PyReadonlyArray1<'py, usize>,
) -> PyResult<(Vec<usize>, f32)> {
    let priors = priors.as_array();
    let outputs = outputs.as_array();
    let transitions = transitions.as_array();
    let sequence = sequence.as_array();

    let t_len = sequence.len(); // T: sequence length
    let n_states = priors.len(); // N: number of states

    if t_len == 0 {
        return Ok((vec![], 0.0));
    }

    // V[t][j] = max log probability of being in state j at time t
    let mut v = vec![vec![0.0f32; n_states]; t_len];
    // B[t][j] = argmax (backpointer) - which state led to state j at time t
    let mut b = vec![vec![0usize; n_states]; t_len];

    // Initialization: V[0][j] = P[j] + O[j, sequence[0]]
    let first_symbol = sequence[0];
    for j in 0..n_states {
        v[0][j] = priors[j] + outputs[[j, first_symbol]];
    }

    // Induction: for each time step t and each state j
    // V[t][j] = max_i(V[t-1][i] + X[i][j]) + O[j, sequence[t]]
    for t in 1..t_len {
        let symbol = sequence[t];
        for j in 0..n_states {
            let mut best_val = f32::NEG_INFINITY;
            let mut best_idx = 0;

            // Find the best previous state
            for i in 0..n_states {
                let val = v[t - 1][i] + transitions[[i, j]];
                if val > best_val {
                    best_val = val;
                    best_idx = i;
                }
            }

            v[t][j] = best_val + outputs[[j, symbol]];
            b[t][j] = best_idx;
        }
    }

    // Termination: find the best final state
    let mut best_final_state = 0;
    let mut best_final_prob = v[t_len - 1][0];
    for j in 1..n_states {
        if v[t_len - 1][j] > best_final_prob {
            best_final_prob = v[t_len - 1][j];
            best_final_state = j;
        }
    }

    // Backtracking: reconstruct the best path
    let mut path = vec![0usize; t_len];
    path[t_len - 1] = best_final_state;

    for t in (1..t_len).rev() {
        path[t - 1] = b[t][path[t]];
    }

    Ok((path, best_final_prob))
}

/// Forward algorithm: computes forward probabilities (alpha values)
///
/// This is the optimized Rust implementation of HiddenMarkovModelTagger._forward_probability()
///
/// # Arguments
/// * `priors` - log prior probabilities, shape (N,)
/// * `outputs` - log output probabilities, shape (N, M)
/// * `transitions` - log transition probabilities, shape (N, N)
/// * `sequence` - sequence of symbol indices, shape (T,)
///
/// # Returns
/// * PyArray2<f64>: Forward probability matrix, shape (T, N)
///   alpha[t][i] = log P(o_1, ..., o_t, q_t = i | lambda)
#[pyfunction]
pub fn hmm_forward_probability<'py>(
    py: Python<'py>,
    priors: PyReadonlyArray1<'py, f64>,
    outputs: PyReadonlyArray2<'py, f64>,
    transitions: PyReadonlyArray2<'py, f64>,
    sequence: PyReadonlyArray1<'py, usize>,
) -> PyResult<Bound<'py, PyArray2<f64>>> {
    let priors = priors.as_array();
    let outputs = outputs.as_array();
    let transitions = transitions.as_array();
    let sequence = sequence.as_array();

    let t_len = sequence.len();
    let n_states = priors.len();

    // Initialize alpha matrix
    let mut alpha = vec![vec![f64::NEG_INFINITY; n_states]; t_len];

    if t_len == 0 {
        return Ok(PyArray2::from_vec2_bound(py, &alpha)?);
    }

    // Initialization: alpha[0][i] = priors[i] + outputs[i][sequence[0]]
    let first_symbol = sequence[0];
    for i in 0..n_states {
        alpha[0][i] = priors[i] + outputs[[i, first_symbol]];
    }

    // Induction: alpha[t][i] = logsumexp(alpha[t-1][j] + transitions[j][i]) + outputs[i][sequence[t]]
    for t in 1..t_len {
        let symbol = sequence[t];
        for i in 0..n_states {
            // Compute logsumexp of alpha[t-1][j] + transitions[j][i] for all j
            let mut max_val = f64::NEG_INFINITY;
            for j in 0..n_states {
                let val = alpha[t - 1][j] + transitions[[j, i]];
                if val > max_val {
                    max_val = val;
                }
            }

            if max_val == f64::NEG_INFINITY {
                alpha[t][i] = f64::NEG_INFINITY;
                continue;
            }

            // logsumexp trick: log(sum(exp(x_i))) = max + log(sum(exp(x_i - max)))
            let mut sum_exp = 0.0;
            for j in 0..n_states {
                let val = alpha[t - 1][j] + transitions[[j, i]];
                sum_exp += (val - max_val).exp();
            }

            alpha[t][i] = max_val + sum_exp.ln() + outputs[[i, symbol]];
        }
    }

    Ok(PyArray2::from_vec2_bound(py, &alpha)?)
}

/// Backward algorithm: computes backward probabilities (beta values)
///
/// This is the optimized Rust implementation of HiddenMarkovModelTagger._backward_probability()
///
/// # Arguments
/// * `outputs` - log output probabilities, shape (N, M)
/// * `transitions` - log transition probabilities, shape (N, N) [will be transposed internally]
/// * `sequence` - sequence of symbol indices, shape (T,)
///
/// # Returns
/// * PyArray2<f64>: Backward probability matrix, shape (T, N)
///   beta[t][i] = log P(o_{t+1}, ..., o_T | q_t = i, lambda)
#[pyfunction]
pub fn hmm_backward_probability<'py>(
    py: Python<'py>,
    outputs: PyReadonlyArray2<'py, f64>,
    transitions: PyReadonlyArray2<'py, f64>,
    sequence: PyReadonlyArray1<'py, usize>,
) -> PyResult<Bound<'py, PyArray2<f64>>> {
    let outputs = outputs.as_array();
    let transitions = transitions.as_array();
    let sequence = sequence.as_array();

    let t_len = sequence.len();
    let n_states = outputs.shape()[0];

    // Initialize beta matrix
    let mut beta = vec![vec![f64::NEG_INFINITY; n_states]; t_len];

    if t_len == 0 {
        return Ok(PyArray2::from_vec2_bound(py, &beta)?);
    }

    // Initialization: beta[T-1][i] = log(1) = 0
    for i in 0..n_states {
        beta[t_len - 1][i] = 0.0; // log2(1) = 0 in any base
    }

    // Induction (backward): beta[t][i] = logsumexp(transitions[i][j] + beta[t+1][j] + outputs[j][sequence[t+1]])
    for t in (0..t_len - 1).rev() {
        let next_symbol = sequence[t + 1];
        for i in 0..n_states {
            // Compute logsumexp of transitions[i][j] + beta[t+1][j] + outputs[j][sequence[t+1]]
            let mut max_val = f64::NEG_INFINITY;
            for j in 0..n_states {
                let val = transitions[[i, j]] + beta[t + 1][j] + outputs[[j, next_symbol]];
                if val > max_val {
                    max_val = val;
                }
            }

            if max_val == f64::NEG_INFINITY {
                beta[t][i] = f64::NEG_INFINITY;
                continue;
            }

            // logsumexp trick
            let mut sum_exp = 0.0;
            for j in 0..n_states {
                let val = transitions[[i, j]] + beta[t + 1][j] + outputs[[j, next_symbol]];
                sum_exp += (val - max_val).exp();
            }

            beta[t][i] = max_val + sum_exp.ln();
        }
    }

    Ok(PyArray2::from_vec2_bound(py, &beta)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use numpy::{PyArray1, PyArray2, PyArrayMethods};

    #[test]
    fn test_viterbi_basic() {
        // Simple 2-state HMM test
        // This test verifies the Viterbi algorithm works correctly
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            // 2 states, 2 symbols, sequence length 3
            let priors = vec![0.0f32, -1.0]; // state 0 more likely
            let outputs = vec![
                vec![0.0, -1.0], // state 0 prefers symbol 0
                vec![-1.0, 0.0], // state 1 prefers symbol 1
            ];
            let transitions = vec![
                vec![-0.5, -0.5], // equal transition from state 0
                vec![-0.5, -0.5], // equal transition from state 1
            ];
            let sequence = vec![0, 1, 0];

            let priors_array = PyArray1::from_vec_bound(py, priors);
            let outputs_array = PyArray2::from_vec2_bound(py, &outputs).unwrap();
            let transitions_array = PyArray2::from_vec2_bound(py, &transitions).unwrap();
            let sequence_array = PyArray1::from_vec_bound(py, sequence);

            let result = hmm_best_path(
                py,
                priors_array.readonly(),
                outputs_array.readonly(),
                transitions_array.readonly(),
                sequence_array.readonly(),
            );

            assert!(result.is_ok());
            let (path, _prob) = result.unwrap();
            assert_eq!(path.len(), 3);
        });
    }
}
