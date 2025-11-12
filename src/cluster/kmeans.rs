// Natural Language Toolkit: K-Means Clustering (Rust Implementation)
// Optimized implementations of K-means clustering algorithms

use numpy::{PyArray1, PyArray2, PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;

/// Classify a vector to the nearest cluster centroid
///
/// This is the optimized Rust implementation of KMeansClusterer.classify_vectorspace()
///
/// # Arguments
/// * `vector` - The vector to classify, shape (d,) where d = dimensions
/// * `means` - The cluster centroids, shape (k, d) where k = number of clusters
/// * `distance_fn` - Python callable that computes distance between two vectors
///
/// # Returns
/// * usize: Index of the closest cluster (0 to k-1)
#[pyfunction]
pub fn kmeans_classify_vectorspace(
    py: Python<'_>,
    vector: PyReadonlyArray1<'_, f64>,
    means: PyReadonlyArray2<'_, f64>,
    distance_fn: PyObject,
) -> PyResult<usize> {
    let vector = vector.as_array();
    let means = means.as_array();

    let k = means.shape()[0]; // number of clusters
    let _d = vector.len(); // number of dimensions

    if k == 0 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "No means provided",
        ));
    }

    let mut best_index = 0;
    let mut best_distance = f64::INFINITY;

    // Find the closest centroid
    for i in 0..k {
        let mean_slice = means.row(i);

        // Convert to Python arrays for distance function call
        let vec_py = PyArray1::from_slice_bound(py, vector.as_slice().unwrap());
        let mean_py = PyArray1::from_slice_bound(py, mean_slice.as_slice().unwrap());

        // Call the Python distance function
        let dist: f64 = distance_fn
            .call1(py, (vec_py, mean_py))?
            .extract(py)?;

        if dist < best_distance {
            best_distance = dist;
            best_index = i;
        }
    }

    Ok(best_index)
}

/// Perform one iteration of K-means clustering
///
/// This is the optimized Rust implementation of KMeansClusterer._cluster_vectorspace()
/// for a single iteration (assignment + update step)
///
/// # Arguments
/// * `vectors` - All data vectors, shape (n, d)
/// * `means` - Current cluster centroids, shape (k, d)
/// * `distance_fn` - Python callable for distance computation
/// * `avoid_empty_clusters` - Whether to include old mean when computing new centroid
///
/// # Returns
/// * Tuple of:
///   - PyArray2<f64>: New means, shape (k, d)
///   - Vec<Vec<usize>>: Cluster assignments (list of vector indices for each cluster)
///   - f64: Total distance change (for convergence checking)
#[pyfunction]
#[pyo3(signature = (vectors, means, distance_fn, avoid_empty_clusters=false))]
pub fn kmeans_iteration<'py>(
    py: Python<'py>,
    vectors: PyReadonlyArray2<'py, f64>,
    means: PyReadonlyArray2<'py, f64>,
    distance_fn: PyObject,
    avoid_empty_clusters: bool,
) -> PyResult<(Bound<'py, PyArray2<f64>>, Vec<Vec<usize>>, f64)> {
    let vectors = vectors.as_array();
    let means = means.as_array();

    let n = vectors.shape()[0]; // number of vectors
    let k = means.shape()[0]; // number of clusters
    let d = vectors.shape()[1]; // dimensions

    // Step 1: Assign vectors to clusters
    let mut clusters: Vec<Vec<usize>> = vec![Vec::new(); k];

    for i in 0..n {
        let vector = vectors.row(i);
        let vec_py = PyArray1::from_slice_bound(py, vector.as_slice().unwrap());

        // Find closest mean
        let mut best_cluster = 0;
        let mut best_distance = f64::INFINITY;

        for j in 0..k {
            let mean = means.row(j);
            let mean_py = PyArray1::from_slice_bound(py, mean.as_slice().unwrap());
            let dist: f64 = distance_fn.call1(py, (vec_py.clone(), mean_py))?.extract(py)?;

            if dist < best_distance {
                best_distance = dist;
                best_cluster = j;
            }
        }

        clusters[best_cluster].push(i);
    }

    // Step 2: Compute new centroids
    let mut new_means = vec![vec![0.0; d]; k];
    let mut total_distance = 0.0;

    for j in 0..k {
        let cluster = &clusters[j];

        if cluster.is_empty() {
            if avoid_empty_clusters {
                // Include the old mean in the computation
                new_means[j] = means.row(j).to_vec();
            } else {
                // Keep the old mean (shouldn't happen in practice with good initialization)
                new_means[j] = means.row(j).to_vec();
            }
        } else {
            // Compute centroid as mean of all vectors in cluster
            let mut centroid = vec![0.0; d];
            let divisor = if avoid_empty_clusters {
                cluster.len() + 1
            } else {
                cluster.len()
            } as f64;

            // If avoid_empty_clusters, start with old mean
            if avoid_empty_clusters {
                for dim in 0..d {
                    centroid[dim] = means[[j, dim]];
                }
            }

            // Add all vectors in the cluster
            for &vec_idx in cluster {
                for dim in 0..d {
                    centroid[dim] += vectors[[vec_idx, dim]];
                }
            }

            // Divide by count
            for dim in 0..d {
                centroid[dim] /= divisor;
            }

            new_means[j] = centroid;
        }

        // Compute distance between old and new mean for convergence check
        let old_mean_py = PyArray1::from_slice_bound(py, means.row(j).as_slice().unwrap());
        let new_mean_py = PyArray1::from_vec_bound(py, new_means[j].clone());
        let mean_dist: f64 = distance_fn
            .call1(py, (old_mean_py, new_mean_py))?
            .extract(py)?;
        total_distance += mean_dist;
    }

    let new_means_array = PyArray2::from_vec2_bound(py, &new_means)?;
    Ok((new_means_array, clusters, total_distance))
}

/// Compute the centroid of a cluster of vectors
///
/// This is the optimized Rust implementation of KMeansClusterer._centroid()
///
/// # Arguments
/// * `vectors` - Vectors in the cluster, shape (n, d)
/// * `old_mean` - Previous centroid (for avoid_empty_clusters), shape (d,)
/// * `avoid_empty_clusters` - Whether to include old mean in computation
///
/// # Returns
/// * PyArray1<f64>: The centroid vector, shape (d,)
#[pyfunction]
#[pyo3(signature = (vectors, old_mean=None, avoid_empty_clusters=false))]
pub fn kmeans_centroid<'py>(
    py: Python<'py>,
    vectors: PyReadonlyArray2<'py, f64>,
    old_mean: Option<PyReadonlyArray1<'py, f64>>,
    avoid_empty_clusters: bool,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let vectors = vectors.as_array();
    let n = vectors.shape()[0];
    let d = vectors.shape()[1];

    if n == 0 && !avoid_empty_clusters {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Cannot compute centroid of empty cluster without avoid_empty_clusters=True",
        ));
    }

    let mut centroid = vec![0.0; d];
    let mut count = 0.0;

    // If avoid_empty_clusters, start with the old mean
    if avoid_empty_clusters {
        if let Some(old_mean) = old_mean {
            let old_mean = old_mean.as_array();
            for i in 0..d {
                centroid[i] = old_mean[i];
            }
            count = 1.0;
        }
    }

    // Add all vectors
    for i in 0..n {
        for j in 0..d {
            centroid[j] += vectors[[i, j]];
        }
        count += 1.0;
    }

    // Compute mean
    for i in 0..d {
        centroid[i] /= count;
    }

    Ok(PyArray1::from_vec_bound(py, centroid))
}

/// Fast euclidean distance computation (optimized)
///
/// # Arguments
/// * `vec1` - First vector, shape (d,)
/// * `vec2` - Second vector, shape (d,)
///
/// # Returns
/// * f64: Euclidean distance
#[pyfunction]
pub fn euclidean_distance(
    vec1: PyReadonlyArray1<'_, f64>,
    vec2: PyReadonlyArray1<'_, f64>,
) -> PyResult<f64> {
    let vec1 = vec1.as_array();
    let vec2 = vec2.as_array();

    if vec1.len() != vec2.len() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Vectors must have same length",
        ));
    }

    let mut sum = 0.0;
    for i in 0..vec1.len() {
        let diff = vec1[i] - vec2[i];
        sum += diff * diff;
    }

    Ok(sum.sqrt())
}

/// Fast cosine distance computation (optimized)
///
/// # Arguments
/// * `vec1` - First vector, shape (d,)
/// * `vec2` - Second vector, shape (d,)
///
/// # Returns
/// * f64: Cosine distance (1 - cosine_similarity)
#[pyfunction]
pub fn cosine_distance(
    vec1: PyReadonlyArray1<'_, f64>,
    vec2: PyReadonlyArray1<'_, f64>,
) -> PyResult<f64> {
    let vec1 = vec1.as_array();
    let vec2 = vec2.as_array();

    if vec1.len() != vec2.len() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Vectors must have same length",
        ));
    }

    let mut dot_product = 0.0;
    let mut norm1 = 0.0;
    let mut norm2 = 0.0;

    for i in 0..vec1.len() {
        dot_product += vec1[i] * vec2[i];
        norm1 += vec1[i] * vec1[i];
        norm2 += vec2[i] * vec2[i];
    }

    if norm1 == 0.0 || norm2 == 0.0 {
        return Ok(1.0); // Maximum distance for zero vectors
    }

    let cosine_sim = dot_product / (norm1.sqrt() * norm2.sqrt());
    Ok(1.0 - cosine_sim)
}

#[cfg(test)]
mod tests {
    use super::*;
    use numpy::{PyArray1, PyArrayMethods};

    #[test]
    fn test_euclidean_distance() {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let vec1 = PyArray1::from_vec_bound(py, vec![0.0, 0.0]);
            let vec2 = PyArray1::from_vec_bound(py, vec![3.0, 4.0]);

            let dist = euclidean_distance(vec1.readonly(), vec2.readonly()).unwrap();
            assert!((dist - 5.0).abs() < 1e-10);
        });
    }

    #[test]
    fn test_cosine_distance() {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let vec1 = PyArray1::from_vec_bound(py, vec![1.0, 0.0]);
            let vec2 = PyArray1::from_vec_bound(py, vec![0.0, 1.0]);

            let dist = cosine_distance(vec1.readonly(), vec2.readonly()).unwrap();
            assert!((dist - 1.0).abs() < 1e-10); // Orthogonal vectors
        });
    }
}
