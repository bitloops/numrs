import numpy as np
import time

def run_benchmark(iterations=20):
    print("\nRunning NumPy Benchmark with larger arrays and more complex operations")
    print("================================================================")

    # Create larger arrays for more meaningful benchmarks
    size = 100000
    arr1 = np.arange(size)
    arr2 = np.arange(size) * 2

    print(f"Created arrays with {size} elements each")

    # Test array properties
    print("\nArray properties:")
    print("Array 1 shape:", arr1.shape)
    print("Array 1 size:", arr1.size)
    print("Array 1 ndim:", arr1.ndim)
    print("Array 1 dtype:", arr1.dtype)

    # Test computationally intensive operations
    print("\nRunning computationally intensive operations...")

    # Test 1: Vector addition (element-wise)
    start_time = time.time()
    result = None
    for _ in range(iterations):
        result = arr1 + arr2
    end_time = time.time()
    add_duration = end_time - start_time

    # Test 2: Scalar operations
    start_time = time.time()
    for _ in range(iterations):
        result = arr1 + 10.5
    end_time = time.time()
    scalar_duration = end_time - start_time

    # Test 3: Multiple operations (equivalent to method chaining)
    start_time = time.time()
    for _ in range(iterations):
        result = arr1 + arr2 + 5.0 + arr1 + 2.5 + arr2
    end_time = time.time()
    multi_duration = end_time - start_time

    # Test 4: Complex operations with multiple arrays
    start_time = time.time()
    for _ in range(iterations):
        # Create a new array for each iteration to simulate more complex operations
        temp_arr = np.arange(size) * 3
        result = arr1 + arr2 + 5.0 + temp_arr + 2.5
    end_time = time.time()
    complex_duration = end_time - start_time

    # Print benchmark results
    print("\nBenchmark Results:")
    print(f"Vector addition ({iterations} iterations): {add_duration:.4f} seconds")
    print(f"Average time per addition: {(add_duration * 1000000 / iterations):.2f} microseconds")

    print(f"\nScalar operations ({iterations} iterations): {scalar_duration:.4f} seconds")
    print(f"Average time per scalar operation: {(scalar_duration * 1000000 / iterations):.2f} microseconds")

    print(f"\nMultiple operations ({iterations} iterations): {multi_duration:.4f} seconds")
    print(f"Average time per multiple operation: {(multi_duration * 1000000 / iterations):.2f} microseconds")

    print(f"\nComplex operations ({iterations} iterations): {complex_duration:.4f} seconds")
    print(f"Average time per complex operation: {(complex_duration * 1000000 / iterations):.2f} microseconds")

    # Print sample results
    print("\nSample results:")
    print("First 5 elements of result:", result[0], result[1], result[2], result[3], result[4])

if __name__ == "__main__":
    run_benchmark()
