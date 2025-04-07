import numpy as np
import time

def run_benchmark(iterations=1000000):
    start_time = time.time()

    # Create arrays
    arr1 = np.array([1, 2, 3], dtype=np.float64)
    arr2 = np.array([4, 5, 6], dtype=np.float64)

    for _ in range(iterations):
        # Array properties
        shape = arr1.shape
        size = arr1.size
        ndim = arr1.ndim
        dtype = arr1.dtype

        # Element access
        elem = arr1[1]

        # Array operations
        sum_array = arr1 + arr2
        scalar_sum = arr1 + 10

    end_time = time.time()
    duration = end_time - start_time

    # Print results once
    print(f"NumPy Results (first iteration):")
    print(f"Array 1 shape: {arr1.shape}")
    print(f"Array 1 size: {arr1.size}")
    print(f"Array 1 ndim: {arr1.ndim}")
    print(f"Array 1 dtype: {arr1.dtype}")
    print(f"Element at index 1: {arr1[1]}")
    print(f"Sum of arrays: {arr1 + arr2}")
    print(f"Array + scalar: {arr1 + 10}")
    print(f"\nBenchmark completed in {duration:.4f} seconds")
    print(f"Average time per iteration: {(duration/iterations)*1000000:.2f} microseconds")

if __name__ == "__main__":
    run_benchmark()
