const { NdArray } = require('./index')

function runBenchmark(iterations = 20) {
    console.log("\nRunning Rust NdArray Benchmark with larger arrays and more complex operations")
    console.log("================================================================")

    // Create larger arrays for more meaningful benchmarks
    const size = 100000
    const arr1 = new NdArray(Array(size).fill(0).map((_, i) => i))
    const arr2 = new NdArray(Array(size).fill(0).map((_, i) => i * 2))

    console.log(`Created arrays with ${size} elements each`)

    // Test array properties
    console.log("\nArray properties:")
    console.log("Array 1 shape:", arr1.shape)
    console.log("Array 1 size:", arr1.size)
    console.log("Array 1 ndim:", arr1.ndim)
    console.log("Array 1 dtype:", arr1.dtype)

    // Test computationally intensive operations
    console.log("\nRunning computationally intensive operations...")

    // Test 1: Vector addition (element-wise)
    const addStartTime = process.hrtime.bigint()
    let result
    for (let i = 0; i < iterations; i++) {
        result = arr1.add(arr2)
    }
    const addEndTime = process.hrtime.bigint()
    const addDuration = Number(addEndTime - addStartTime) / 1e9

    // Test 2: Scalar operations
    const scalarStartTime = process.hrtime.bigint()
    for (let i = 0; i < iterations; i++) {
        result = arr1.addScalar(10.5)
    }
    const scalarEndTime = process.hrtime.bigint()
    const scalarDuration = Number(scalarEndTime - scalarStartTime) / 1e9

    // Test 3: Method chaining with multiple operations
    const chainStartTime = process.hrtime.bigint()
    for (let i = 0; i < iterations; i++) {
        result = arr1.chain()
            .add(arr2)
            .addScalar(5.0)
            .add(arr1)
            .addScalar(2.5)
            .add(arr2)
    }
    const chainEndTime = process.hrtime.bigint()
    const chainDuration = Number(chainEndTime - chainStartTime) / 1e9

    // Test 4: Complex operations with multiple arrays
    const complexStartTime = process.hrtime.bigint()
    for (let i = 0; i < iterations; i++) {
        // Create a new array for each iteration to simulate more complex operations
        const tempArr = new NdArray(Array(size).fill(0).map((_, i) => i * 3))
        result = arr1.chain()
            .add(arr2)
            .addScalar(5.0)
            .add(tempArr)
            .addScalar(2.5)
    }
    const complexEndTime = process.hrtime.bigint()
    const complexDuration = Number(complexEndTime - complexStartTime) / 1e9

    // Print benchmark results
    console.log("\nBenchmark Results:")
    console.log(`Vector addition (${iterations} iterations): ${addDuration.toFixed(4)} seconds`)
    console.log(`Average time per addition: ${(addDuration * 1000000 / iterations).toFixed(2)} microseconds`)

    console.log(`\nScalar operations (${iterations} iterations): ${scalarDuration.toFixed(4)} seconds`)
    console.log(`Average time per scalar operation: ${(scalarDuration * 1000000 / iterations).toFixed(2)} microseconds`)

    console.log(`\nMethod chaining (${iterations} iterations): ${chainDuration.toFixed(4)} seconds`)
    console.log(`Average time per chain operation: ${(chainDuration * 1000000 / iterations).toFixed(2)} microseconds`)

    console.log(`\nComplex operations (${iterations} iterations): ${complexDuration.toFixed(4)} seconds`)
    console.log(`Average time per complex operation: ${(complexDuration * 1000000 / iterations).toFixed(2)} microseconds`)

    // Print sample results
    console.log("\nSample results:")
    console.log("First 5 elements of result:",
        result.get([0]), result.get([1]), result.get([2]), result.get([3]), result.get([4]))
}

runBenchmark()
