#!/usr/bin/env python3
"""
Tests for parallel batch processing functions
"""

import time
import pytest
from nltk_rs import (
    edit_distance, edit_distance_batch,
    jaro_similarity, jaro_similarity_batch,
    jaro_winkler_similarity, jaro_winkler_similarity_batch
)


def test_edit_distance_batch_correctness():
    """Test that batch processing gives correct results"""
    pairs = [
        ("kitten", "sitting"),
        ("saturday", "sunday"),
        ("", ""),
        ("hello", "hello"),
        ("test", "text"),
    ]

    # Compute individually
    individual_results = [edit_distance(s1, s2) for s1, s2 in pairs]

    # Compute in batch
    batch_results = edit_distance_batch(pairs)

    # Should match
    assert individual_results == batch_results
    assert batch_results == [3, 3, 0, 0, 1]


def test_jaro_similarity_batch_correctness():
    """Test that Jaro batch processing gives correct results"""
    pairs = [
        ("martha", "marhta"),
        ("", ""),
        ("abc", "xyz"),
        ("hello", "hello"),
    ]

    # Compute individually
    individual_results = [jaro_similarity(s1, s2) for s1, s2 in pairs]

    # Compute in batch
    batch_results = jaro_similarity_batch(pairs)

    # Should match (within floating point tolerance)
    for ind, batch in zip(individual_results, batch_results):
        assert abs(ind - batch) < 1e-10


def test_jaro_winkler_batch_correctness():
    """Test that Jaro-Winkler batch processing gives correct results"""
    pairs = [
        ("dixon", "dickson"),
        ("martha", "marhta"),
        ("billy", "billy"),
        ("abc", "xyz"),
    ]

    # Compute individually
    individual_results = [jaro_winkler_similarity(s1, s2) for s1, s2 in pairs]

    # Compute in batch
    batch_results = jaro_winkler_similarity_batch(pairs)

    # Should match (within floating point tolerance)
    for ind, batch in zip(individual_results, batch_results):
        assert abs(ind - batch) < 1e-10


def test_edit_distance_batch_performance():
    """Test that batch processing is faster for large inputs"""
    # Create a large list of string pairs
    test_strings = [
        ("the quick brown fox jumps over the lazy dog", "the quack brown fix jumped over a lazy cat"),
        ("artificial intelligence", "artificial int3lligence"),
        ("natural language processing", "nautral language procesing"),
        ("machine learning algorithms", "machine learning algorithims"),
    ] * 100  # 400 pairs

    # Warm up
    edit_distance_batch(test_strings[:10])
    [edit_distance(s1, s2) for s1, s2 in test_strings[:10]]

    # Benchmark individual
    start = time.time()
    individual_results = [edit_distance(s1, s2) for s1, s2 in test_strings]
    individual_time = time.time() - start

    # Benchmark batch
    start = time.time()
    batch_results = edit_distance_batch(test_strings)
    batch_time = time.time() - start

    # Verify correctness
    assert individual_results == batch_results

    # Print performance
    speedup = individual_time / batch_time if batch_time > 0 else float('inf')
    print(f"\n  Individual processing: {individual_time:.4f}s")
    print(f"  Batch processing: {batch_time:.4f}s")
    print(f"  Speedup: {speedup:.2f}x")

    # Batch should be at least as fast (with parallelization, it should be faster)
    # On multi-core systems with many pairs, we expect speedup
    assert batch_time <= individual_time * 1.1  # Allow 10% tolerance


def test_jaro_batch_performance():
    """Test Jaro batch processing performance"""
    test_strings = [
        ("the quick brown fox", "the quack brown fix"),
        ("artificial intelligence", "artificial int3lligence"),
        ("natural language", "nautral language"),
        ("machine learning", "machine learning"),
    ] * 100  # 400 pairs

    # Warm up
    jaro_similarity_batch(test_strings[:10])

    # Benchmark batch
    start = time.time()
    batch_results = jaro_similarity_batch(test_strings)
    batch_time = time.time() - start

    print(f"\n  Jaro batch processing (400 pairs): {batch_time:.4f}s")
    print(f"  Rate: {len(test_strings) / batch_time:.0f} pairs/sec")

    # Verify we got results
    assert len(batch_results) == len(test_strings)
    assert all(0.0 <= r <= 1.0 for r in batch_results)


def test_empty_batch():
    """Test that empty batches work correctly"""
    assert edit_distance_batch([]) == []
    assert jaro_similarity_batch([]) == []
    assert jaro_winkler_similarity_batch([]) == []


def test_single_pair_batch():
    """Test batch processing with a single pair"""
    result = edit_distance_batch([("hello", "world")])
    assert len(result) == 1
    assert result[0] == edit_distance("hello", "world")


if __name__ == "__main__":
    pytest.main([__file__, "-v", "-s"])
