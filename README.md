# Alternating Triplet Partition Algorithm

An optimized pseudo-polynomial time algorithm that leverages a deterministic distribution sieve paired with a dynamic programming subset-sum backend to balance elements into two equal-weight sets.

## Features
- **Alternating Sieve:** Sifts input arrays in $3N$ triplet blocks to pre-balance columns.
- **Dynamic Programming Engine:** Searches the remainder pool in $O(N \cdot T)$ time, bypassing classic $O(2^N)$ combinatorial walls for dense datasets.
- **High-Performance Verification:** Sub-millisecond execution times for standard real-world applications.

## Usage
Simply run the core execution script using Python 3:
```bash
python3 script.py
```