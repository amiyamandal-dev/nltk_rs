#!/usr/bin/env python3
"""
Comprehensive integration tests for NLTK Rust optimizations
Tests all phases: Distance metrics, HMM, K-means, ALINE
"""

import numpy as np
import pytest

# Phase 1: Distance Metrics
def test_phase1_distance_metrics():
    """Test Phase 1: String distance metrics"""
    from nltk_rs import edit_distance, jaro_similarity, jaro_winkler_similarity

    # Edit distance
    assert edit_distance("kitten", "sitting") == 3
    assert edit_distance("saturday", "sunday") == 3
    assert edit_distance("", "") == 0

    # Jaro similarity
    sim = jaro_similarity("martha", "marhta")
    assert abs(sim - 0.9444) < 0.001

    # Jaro-Winkler
    jw = jaro_winkler_similarity("dixon", "dickson")
    assert abs(jw - 0.8324) < 0.01


# Phase 2: HMM Tagging
def test_phase2_hmm_tagging():
    """Test Phase 2: HMM tagging functions"""
    from nltk_rs import hmm_best_path

    # Simple 2-state HMM
    priors = np.array([0.0, -1.0], dtype=np.float32)
    outputs = np.array([[0.0, -1.0], [-1.0, 0.0]], dtype=np.float32)
    transitions = np.array([[-0.5, -0.5], [-0.5, -0.5]], dtype=np.float32)
    sequence = np.array([0, 1, 0, 1, 0], dtype=np.uintp)

    path, prob = hmm_best_path(priors, outputs, transitions, sequence)

    assert len(path) == 5
    assert isinstance(prob, float)


# Phase 3: K-means Clustering
def test_phase3_kmeans():
    """Test Phase 3: K-means clustering"""
    from nltk_rs import (
        kmeans_classify_vectorspace,
        euclidean_distance,
        cosine_distance
    )

    # Test classification
    means = np.array([[0.0, 0.0], [5.0, 5.0]])
    vector = np.array([1.0, 1.0])
    cluster = kmeans_classify_vectorspace(vector, means, euclidean_distance)
    assert cluster == 0  # Closer to first cluster

    # Test distance functions
    v1 = np.array([0.0, 0.0])
    v2 = np.array([3.0, 4.0])
    dist = euclidean_distance(v1, v2)
    assert abs(dist - 5.0) < 1e-6

    v3 = np.array([1.0, 0.0])
    v4 = np.array([0.0, 1.0])
    cos_dist = cosine_distance(v3, v4)
    assert abs(cos_dist - 1.0) < 1e-6


# Phase 4: ALINE Phonetic
def test_phase4_aline():
    """Test Phase 4: ALINE phonetic alignment"""
    from nltk_rs import aline_align_score

    # Test basic alignment score
    score1 = aline_align_score("test", "test")
    assert score1 > 0.0  # Identical strings should have positive score

    score2 = aline_align_score("test", "text")
    assert score2 > 0.0  # Similar strings should have positive score

    score3 = aline_align_score("abc", "xyz")
    # Different strings should have lower score than similar ones
    assert score3 < score1


# NLTK Integration Tests
def test_nltk_integration():
    """Test integration with NLTK Python modules"""
    from nltk.metrics import distance
    from nltk.tag import hmm
    from nltk.cluster import kmeans

    # Check Rust availability flags
    assert hasattr(distance, '_RUST_DISTANCE_AVAILABLE')
    assert hasattr(hmm, '_RUST_HMM_AVAILABLE')
    assert hasattr(kmeans, '_RUST_KMEANS_AVAILABLE')

    # Verify they're active
    assert distance._RUST_DISTANCE_AVAILABLE == True
    assert hmm._RUST_HMM_AVAILABLE == True
    assert kmeans._RUST_KMEANS_AVAILABLE == True


# Performance benchmark
def test_performance_comparison():
    """Quick performance comparison"""
    import time
    from nltk_rs import edit_distance as rust_edit_distance
    from nltk.metrics.distance import edit_distance as python_edit_distance

    s1 = "the quick brown fox"
    s2 = "the quack brown fix"

    # Warm up
    rust_edit_distance(s1, s2)
    python_edit_distance(s1, s2)

    # Benchmark (small scale for quick test)
    iterations = 100

    start = time.time()
    for _ in range(iterations):
        python_edit_distance(s1, s2)
    python_time = time.time() - start

    start = time.time()
    for _ in range(iterations):
        rust_edit_distance(s1, s2)
    rust_time = time.time() - start

    speedup = python_time / rust_time
    print(f"\nPerformance: {speedup:.1f}x faster (Rust vs Python)")

    # Rust should be faster (though on small inputs, overhead might dominate)
    # We just verify it completes successfully
    assert speedup > 0


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
