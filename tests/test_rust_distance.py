#!/usr/bin/env python3
"""
Test script to compare Rust implementations with Python versions
and benchmark performance.
"""

import time
from nltk.metrics.distance import (
    edit_distance as py_edit_distance,
    jaro_similarity as py_jaro_similarity,
    jaro_winkler_similarity as py_jaro_winkler_similarity,
)
from nltk_rs import (
    edit_distance as rs_edit_distance,
    jaro_similarity as rs_jaro_similarity,
    jaro_winkler_similarity as rs_jaro_winkler_similarity,
)


def test_correctness():
    """Test that Rust implementations produce the same results as Python."""
    print("=" * 70)
    print("CORRECTNESS TESTS")
    print("=" * 70)

    # Test cases from the NLTK demo
    test_cases = [
        ("rain", "shine"),
        ("abcdef", "acbdef"),
        ("language", "lnaguaeg"),
        ("language", "lnaugage"),
        ("language", "lngauage"),
        ("saturday", "sunday"),
        ("martha", "marhta"),
        ("billy", "billy"),
        ("billy", "bill"),
        ("dwayne", "duane"),
        ("dixon", "dickson"),
        ("", ""),
        ("hello", "hello"),
    ]

    all_passed = True

    print("\n1. Edit Distance Tests:")
    print("-" * 70)
    for s1, s2 in test_cases:
        py_result = py_edit_distance(s1, s2)
        rs_result = rs_edit_distance(s1, s2)
        match = "✓" if py_result == rs_result else "✗"
        if py_result != rs_result:
            all_passed = False
        print(f"{match} '{s1}' -> '{s2}': Python={py_result}, Rust={rs_result}")

    print("\n2. Edit Distance with Transpositions:")
    print("-" * 70)
    transposition_cases = [
        ("abcdef", "acbdef"),
        ("language", "lnaguage"),
    ]
    for s1, s2 in transposition_cases:
        py_result = py_edit_distance(s1, s2, transpositions=True)
        rs_result = rs_edit_distance(s1, s2, transpositions=True)
        match = "✓" if py_result == rs_result else "✗"
        if py_result != rs_result:
            all_passed = False
        print(f"{match} '{s1}' -> '{s2}': Python={py_result}, Rust={rs_result}")

    print("\n3. Jaro Similarity Tests:")
    print("-" * 70)
    for s1, s2 in test_cases:
        py_result = py_jaro_similarity(s1, s2)
        rs_result = rs_jaro_similarity(s1, s2)
        match = "✓" if abs(py_result - rs_result) < 0.0001 else "✗"
        if abs(py_result - rs_result) >= 0.0001:
            all_passed = False
        print(f"{match} '{s1}' -> '{s2}': Python={py_result:.4f}, Rust={rs_result:.4f}")

    print("\n4. Jaro-Winkler Similarity Tests:")
    print("-" * 70)
    for s1, s2 in test_cases:
        py_result = py_jaro_winkler_similarity(s1, s2)
        rs_result = rs_jaro_winkler_similarity(s1, s2)
        match = "✓" if abs(py_result - rs_result) < 0.0001 else "✗"
        if abs(py_result - rs_result) >= 0.0001:
            all_passed = False
        print(f"{match} '{s1}' -> '{s2}': Python={py_result:.4f}, Rust={rs_result:.4f}")

    print("\n" + "=" * 70)
    if all_passed:
        print("✓ ALL TESTS PASSED!")
    else:
        print("✗ SOME TESTS FAILED!")
    print("=" * 70)

    return all_passed


def benchmark():
    """Benchmark Rust vs Python implementations."""
    print("\n\n" + "=" * 70)
    print("PERFORMANCE BENCHMARKS")
    print("=" * 70)

    # Test data - various string lengths
    test_data = [
        ("short", "pairs"),
        ("medium_length_string", "another_medium_one"),
        ("this_is_a_longer_string_for_testing", "this_is_another_longer_string"),
        ("supercalifragilisticexpialidocious", "antidisestablishmentarianism"),
    ]

    iterations = 1000

    print(f"\nRunning {iterations} iterations per test...")

    # Edit Distance Benchmark
    print("\n1. Edit Distance:")
    print("-" * 70)
    for s1, s2 in test_data:
        # Python
        start = time.perf_counter()
        for _ in range(iterations):
            py_edit_distance(s1, s2)
        py_time = time.perf_counter() - start

        # Rust
        start = time.perf_counter()
        for _ in range(iterations):
            rs_edit_distance(s1, s2)
        rs_time = time.perf_counter() - start

        speedup = py_time / rs_time if rs_time > 0 else float("inf")
        print(f"  '{s1[:20]}...' -> '{s2[:20]}...'")
        print(f"    Python: {py_time*1000:.2f}ms | Rust: {rs_time*1000:.2f}ms | Speedup: {speedup:.2f}x")

    # Jaro Similarity Benchmark
    print("\n2. Jaro Similarity:")
    print("-" * 70)
    for s1, s2 in test_data:
        # Python
        start = time.perf_counter()
        for _ in range(iterations):
            py_jaro_similarity(s1, s2)
        py_time = time.perf_counter() - start

        # Rust
        start = time.perf_counter()
        for _ in range(iterations):
            rs_jaro_similarity(s1, s2)
        rs_time = time.perf_counter() - start

        speedup = py_time / rs_time if rs_time > 0 else float("inf")
        print(f"  '{s1[:20]}...' -> '{s2[:20]}...'")
        print(f"    Python: {py_time*1000:.2f}ms | Rust: {rs_time*1000:.2f}ms | Speedup: {speedup:.2f}x")

    # Jaro-Winkler Similarity Benchmark
    print("\n3. Jaro-Winkler Similarity:")
    print("-" * 70)
    for s1, s2 in test_data:
        # Python
        start = time.perf_counter()
        for _ in range(iterations):
            py_jaro_winkler_similarity(s1, s2)
        py_time = time.perf_counter() - start

        # Rust
        start = time.perf_counter()
        for _ in range(iterations):
            rs_jaro_winkler_similarity(s1, s2)
        rs_time = time.perf_counter() - start

        speedup = py_time / rs_time if rs_time > 0 else float("inf")
        print(f"  '{s1[:20]}...' -> '{s2[:20]}...'")
        print(f"    Python: {py_time*1000:.2f}ms | Rust: {rs_time*1000:.2f}ms | Speedup: {speedup:.2f}x")

    print("\n" + "=" * 70)


if __name__ == "__main__":
    # Run correctness tests
    passed = test_correctness()

    if passed:
        # Only run benchmarks if correctness tests pass
        benchmark()
    else:
        print("\n⚠️  Skipping benchmarks due to correctness test failures.")
