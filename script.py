import time


def solve_two_pile_partition(S):
    # Ensure standard 3N compliance via zero-padding if needed
    working_S = list(S)
    remainder = len(working_S) % 3
    if remainder != 0:
        working_S.extend([0] * (3 - remainder))

    N = len(working_S) // 3
    pile_A, pile_B, pile_C = [], [], []

    # 1. The Triplet Distribution Sieve
    for i in range(1, N + 1):
        idx = (i - 1) * 3
        triplet = sorted(working_S[idx:idx + 3])  # [Small, Medium, Large]

        small, medium, large = triplet[0], triplet[1], triplet[2]

        if i % 2 == 0:
            pile_A.append(large)
            pile_B.append(small)
            pile_C.append(medium)
        else:
            pile_A.append(small)
            pile_B.append(large)
            pile_C.append(medium)

    sum_A, sum_B, sum_C = sum(pile_A), sum(pile_B), sum(pile_C)
    target_doubled = sum_B + sum_C - sum_A

    if target_doubled % 2 != 0:
        return "no", [], []

    target_sum_C_to_A = target_doubled // 2
    if target_sum_C_to_A < 0 or target_sum_C_to_A > sum_C:
        return "no", [], []

    # 2. The Resolution Search (Sub-pile generator)
    dp = {0: []}
    for item in pile_C:
        current_sums = list(dp.keys())
        for current_sum in current_sums:
            new_sum = current_sum + item
            if new_sum <= target_sum_C_to_A and new_sum not in dp:
                dp[new_sum] = dp[current_sum] + [item]
        if target_sum_C_to_A in dp:
            break

    # 3. Consolidating into the Final Two Combined Piles
    if target_sum_C_to_A in dp:
        c_to_a = dp[target_sum_C_to_A]
        c_to_b = pile_C.copy()
        for x in c_to_a:
            c_to_b.remove(x)

        # Build the final two structural piles
        final_pile_1 = pile_A + c_to_a
        final_pile_2 = pile_B + c_to_b

        return "yes", final_pile_1, final_pile_2

    return "no", [], []


if __name__ == "__main__":
    # Test dataset of rocks
    rocks = [49, 44, 38, 15, 34, 29, 21, 15, 31, 35, 25, 30, 21, 11, 36, 49, 61, 75, 52, 46, 39, 32, 21, 27, 62, 54, 42,
             29, 42, 14, 59, 39, 45, 28, 42, 15, 12, 21, 34, 14, 38, 29, 59, 50, 40, 48, 40, 25, 52, 67, 39, 30, 11, 24,
             10, 20, 27, 55, 40, 66, 31, 20, 25, 47, 59, 66, 27, 35, 14, 34, 19, 24, 37, 17, 30, 29, 46, 41, 60, 36, 45,
             19, 28, 12, 28, 19, 35, 44, 58, 39, 34, 44, 56, 45, 57, 31, 45, 40, 108]

    start = time.perf_counter()
    outcome, side_1, side_2 = solve_two_pile_partition(rocks)
    end = time.perf_counter()

    if outcome == "yes":
        print("[HALT: YES]")
        print(f"Final Combined Pile 1 (Total Weight = {sum(side_1)}):")
        print(f"  Elements: {side_1}\n")
        print(f"Final Combined Pile 2 (Total Weight = {sum(side_2)}):")
        print(f"  Elements: {side_2}\n")
        print(f"Verification: {sum(side_1)} == {sum(side_2)}")
    else:
        print("[HALT: NO]")

    print(f"Search Execution Time: {(end - start) * 1000:.3f} ms")
