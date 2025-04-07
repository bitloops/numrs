const { NdArray } = require('./index')

function runBenchmark(iterations = 1000000) {
    const startTime = process.hrtime.bigint()

    // Create arrays
    const arr1 = new NdArray([1, 2, 3])
    const arr2 = new NdArray([4, 5, 6])

    for (let i = 0; i < iterations; i++) {
        // Array properties
        const shape = arr1.shape
        const size = arr1.size
        const ndim = arr1.ndim
        const dtype = arr1.dtype

        // Element access
        const elem = arr1.get([1])

        // Array operations
        const sumArray = arr1.add(arr2)
        const scalarSum = arr1.addScalar(10)
    }

    const endTime = process.hrtime.bigint()
    const duration = Number(endTime - startTime) / 1e9 // Convert to seconds

    // Print results once
    console.log("Rust NdArray Results (first iteration):")
    console.log("Array 1 shape:", arr1.shape)
    console.log("Array 1 size:", arr1.size)
    console.log("Array 1 ndim:", arr1.ndim)
    console.log("Array 1 dtype:", arr1.dtype)
    console.log("Element at index 1:", arr1.get([1]))
    console.log("Sum of arrays:", arr1.add(arr2).toString())
    console.log("Array + scalar:", arr1.addScalar(10).toString())
    console.log(`\nBenchmark completed in ${duration.toFixed(4)} seconds`)
    console.log(`Average time per iteration: ${(duration * 1000000 / iterations).toFixed(2)} microseconds`)
}

runBenchmark()
