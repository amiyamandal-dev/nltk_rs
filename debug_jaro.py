#!/usr/bin/env python3
"""Debug script to trace Jaro algorithm step by step"""

def jaro_similarity_debug(s1, s2):
    """Jaro similarity with debug output"""
    len_s1, len_s2 = len(s1), len(s2)
    print(f"\nComparing '{s1}' vs '{s2}'")
    print(f"Lengths: {len_s1} vs {len_s2}")

    match_bound = max(len_s1, len_s2) // 2 - 1
    print(f"Match bound: {match_bound}")

    matches = 0
    transpositions = 0
    flagged_1 = []
    flagged_2 = []

    # Find matches
    for i in range(len_s1):
        upperbound = min(i + match_bound, len_s2 - 1)
        lowerbound = max(0, i - match_bound)
        print(f"  i={i} ('{s1[i]}'): checking j={lowerbound} to {upperbound}")

        for j in range(lowerbound, upperbound + 1):
            if s1[i] == s2[j] and j not in flagged_2:
                print(f"    -> Match at j={j} ('{s2[j]}')")
                matches += 1
                flagged_1.append(i)
                flagged_2.append(j)
                break

    print(f"\nMatches: {matches}")
    print(f"flagged_1: {flagged_1}")
    print(f"flagged_2: {flagged_2}")

    if matches == 0:
        return 0

    flagged_2.sort()
    print(f"flagged_2 (sorted): {flagged_2}")

    # Count transpositions
    for i_idx, j_idx in zip(flagged_1, flagged_2):
        if s1[i_idx] != s2[j_idx]:
            print(f"  Transposition: s1[{i_idx}]='{s1[i_idx]}' vs s2[{j_idx}]='{s2[j_idx]}'")
            transpositions += 1

    print(f"Transpositions: {transpositions}")

    jaro = (1 / 3) * (
        matches / len_s1 +
        matches / len_s2 +
        (matches - transpositions // 2) / matches
    )

    print(f"\nCalculation:")
    print(f"  {matches}/{len_s1} + {matches}/{len_s2} + ({matches} - {transpositions}//2)/{matches}")
    print(f"  = {matches/len_s1:.4f} + {matches/len_s2:.4f} + {(matches - transpositions//2)/matches:.4f}")
    print(f"  = {jaro:.4f}")

    return jaro

# Test the failing cases
print("=" * 70)
print("PYTHON JARO ALGORITHM TRACE")
print("=" * 70)

jaro_similarity_debug("saturday", "sunday")
jaro_similarity_debug("language", "lngauage")
