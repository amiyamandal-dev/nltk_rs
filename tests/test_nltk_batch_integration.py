#!/usr/bin/env python3
"""
Test NLTK integration with parallel batch processing functions
"""

import pytest


def test_nltk_batch_functions_available():
    """Test that batch functions are available in NLTK"""
    from nltk.metrics import distance

    # Check that batch functions exist
    assert hasattr(distance, 'edit_distance_batch')
    assert hasattr(distance, 'jaro_similarity_batch')
    assert hasattr(distance, 'jaro_winkler_similarity_batch')

    # Check Rust availability flags
    assert hasattr(distance, '_RUST_BATCH_AVAILABLE')
    assert distance._RUST_BATCH_AVAILABLE == True


def test_edit_distance_batch_via_nltk():
    """Test edit_distance_batch through NLTK API"""
    from nltk.metrics.distance import edit_distance_batch, edit_distance

    pairs = [
        ("kitten", "sitting"),
        ("saturday", "sunday"),
        ("hello", "hello"),
        ("test", "text"),
    ]

    # Test batch function
    batch_results = edit_distance_batch(pairs)
    assert len(batch_results) == len(pairs)
    assert batch_results == [3, 3, 0, 1]

    # Verify matches individual results
    individual_results = [edit_distance(s1, s2) for s1, s2 in pairs]
    assert batch_results == individual_results


def test_jaro_similarity_batch_via_nltk():
    """Test jaro_similarity_batch through NLTK API"""
    from nltk.metrics.distance import jaro_similarity_batch, jaro_similarity

    pairs = [
        ("martha", "marhta"),
        ("dixon", "dickson"),
        ("", ""),
        ("abc", "abc"),
    ]

    # Test batch function
    batch_results = jaro_similarity_batch(pairs)
    assert len(batch_results) == len(pairs)

    # Verify matches individual results
    individual_results = [jaro_similarity(s1, s2) for s1, s2 in pairs]
    for batch, individual in zip(batch_results, individual_results):
        assert abs(batch - individual) < 1e-10


def test_jaro_winkler_batch_via_nltk():
    """Test jaro_winkler_similarity_batch through NLTK API"""
    from nltk.metrics.distance import jaro_winkler_similarity_batch, jaro_winkler_similarity

    pairs = [
        ("dixon", "dickson"),
        ("martha", "marhta"),
        ("billy", "billy"),
    ]

    # Test batch function
    batch_results = jaro_winkler_similarity_batch(pairs)
    assert len(batch_results) == len(pairs)

    # Verify matches individual results
    individual_results = [jaro_winkler_similarity(s1, s2) for s1, s2 in pairs]
    for batch, individual in zip(batch_results, individual_results):
        assert abs(batch - individual) < 1e-10


def test_batch_with_custom_parameters():
    """Test batch functions with custom parameters"""
    from nltk.metrics.distance import edit_distance_batch, jaro_winkler_similarity_batch

    # Edit distance with transpositions
    pairs = [("abcdef", "acbdef"), ("language", "lnaguage")]
    results = edit_distance_batch(pairs, transpositions=True)
    assert results == [1, 1]

    # Jaro-Winkler with custom p and max_l
    pairs = [("dixon", "dickson"), ("martha", "marhta")]
    results = jaro_winkler_similarity_batch(pairs, p=0.15, max_l=3)
    assert len(results) == len(pairs)
    assert all(0.0 <= r <= 1.0 for r in results)


def test_empty_batch():
    """Test batch functions with empty input"""
    from nltk.metrics.distance import (
        edit_distance_batch,
        jaro_similarity_batch,
        jaro_winkler_similarity_batch
    )

    assert edit_distance_batch([]) == []
    assert jaro_similarity_batch([]) == []
    assert jaro_winkler_similarity_batch([]) == []


def test_single_pair_batch():
    """Test batch functions with a single pair"""
    from nltk.metrics.distance import (
        edit_distance_batch,
        jaro_similarity_batch,
        jaro_winkler_similarity_batch,
        edit_distance,
        jaro_similarity,
        jaro_winkler_similarity
    )

    pair = [("hello", "world")]

    # Edit distance
    batch_result = edit_distance_batch(pair)
    individual_result = edit_distance("hello", "world")
    assert batch_result == [individual_result]

    # Jaro similarity
    batch_result = jaro_similarity_batch(pair)
    individual_result = jaro_similarity("hello", "world")
    assert abs(batch_result[0] - individual_result) < 1e-10

    # Jaro-Winkler
    batch_result = jaro_winkler_similarity_batch(pair)
    individual_result = jaro_winkler_similarity("hello", "world")
    assert abs(batch_result[0] - individual_result) < 1e-10


def test_large_batch_performance():
    """Test batch processing with a large number of pairs"""
    import time
    from nltk.metrics.distance import edit_distance_batch, edit_distance

    # Create a large batch
    pairs = [("test", "text"), ("hello", "hallo")] * 200  # 400 pairs

    # Measure batch processing
    start = time.time()
    batch_results = edit_distance_batch(pairs)
    batch_time = time.time() - start

    # Verify correctness
    assert len(batch_results) == len(pairs)
    assert all(r in [1, 2] for r in batch_results)  # Both pairs have distance 1 or 2

    # Measure sequential processing
    start = time.time()
    seq_results = [edit_distance(s1, s2) for s1, s2 in pairs]
    seq_time = time.time() - start

    # Verify results match
    assert batch_results == seq_results

    # Report performance (don't fail if not faster, just report)
    if batch_time > 0:
        speedup = seq_time / batch_time
        print(f"\n  Batch processing speedup: {speedup:.2f}x")
        print(f"  Sequential: {seq_time:.4f}s, Batch: {batch_time:.4f}s")


def test_backward_compatibility():
    """Test that existing NLTK code still works"""
    from nltk.metrics.distance import edit_distance, jaro_similarity, jaro_winkler_similarity

    # These are the existing single-operation functions
    # They should work exactly as before
    assert edit_distance("kitten", "sitting") == 3
    assert abs(jaro_similarity("martha", "marhta") - 0.9444) < 0.001
    assert abs(jaro_winkler_similarity("dixon", "dickson") - 0.8324) < 0.001


if __name__ == "__main__":
    pytest.main([__file__, "-v", "-s"])
