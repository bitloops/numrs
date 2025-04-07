class JsArray {
    constructor(data) {
        this.data = Float64Array.from(data)
    }

    get shape() {
        return [this.data.length]
    }

    get size() {
        return this.data.length
    }

    get ndim() {
        return 1
    }

    get dtype() {
        return 'float64'
    }

    get(indices) {
        return this.data[indices[0]]
    }

    add(other) {
        const result = new Float64Array(this.data.length)
        for (let i = 0; i < this.data.length; i++) {
            result[i] = this.data[i] + other.data[i]
        }
        return new JsArray(result)
    }

    addScalar(scalar) {
        const result = new Float64Array(this.data.length)
        for (let i = 0; i < this.data.length; i++) {
            result[i] = this.data[i] + scalar
        }
        return new JsArray(result)
    }

    toString() {
        return `[${Array.from(this.data).join(', ')}]`
    }
}

function runBenchmark(iterations = 20) {
    console.log("\nRunning Pure JavaScript Array Benchmark with larger arrays and more complex operations")
    console.log("================================================================")

    // Create larger arrays for more meaningful benchmarks
    const size = 100000
    const arr1 = Array(size).fill(0).map((_, i) => i)
    const arr2 = Array(size).fill(0).map((_, i) => i * 2)

    console.log(`Created arrays with ${size} elements each`)

    // Test array properties
    console.log("\nArray properties:")
    console.log("Array 1 length:", arr1.length)

    // Test computationally intensive operations
    console.log("\nRunning computationally intensive operations...")

    // Test 1: Vector addition (element-wise)
    const addStartTime = process.hrtime.bigint()
    let result
    for (let i = 0; i < iterations; i++) {
        result = arr1.map((val, idx) => val + arr2[idx])
    }
    const addEndTime = process.hrtime.bigint()
    const addDuration = Number(addEndTime - addStartTime) / 1e9

    // Test 2: Scalar operations
    const scalarStartTime = process.hrtime.bigint()
    for (let i = 0; i < iterations; i++) {
        result = arr1.map(val => val + 10.5)
    }
    const scalarEndTime = process.hrtime.bigint()
    const scalarDuration = Number(scalarEndTime - scalarStartTime) / 1e9

    // Test 3: Multiple operations (equivalent to method chaining)
    const multiStartTime = process.hrtime.bigint()
    for (let i = 0; i < iterations; i++) {
        result = arr1
            .map((val, idx) => val + arr2[idx])
            .map(val => val + 5.0)
            .map((val, idx) => val + arr1[idx])
            .map(val => val + 2.5)
            .map((val, idx) => val + arr2[idx])
    }
    const multiEndTime = process.hrtime.bigint()
    const multiDuration = Number(multiEndTime - multiStartTime) / 1e9

    // Test 4: Complex operations with multiple arrays
    const complexStartTime = process.hrtime.bigint()
    for (let i = 0; i < iterations; i++) {
        // Create a new array for each iteration to simulate more complex operations
        const tempArr = Array(size).fill(0).map((_, i) => i * 3)
        result = arr1
            .map((val, idx) => val + arr2[idx])
            .map(val => val + 5.0)
            .map((val, idx) => val + tempArr[idx])
            .map(val => val + 2.5)
    }
    const complexEndTime = process.hrtime.bigint()
    const complexDuration = Number(complexEndTime - complexStartTime) / 1e9

    // Print benchmark results
    console.log("\nBenchmark Results:")
    console.log(`Vector addition (${iterations} iterations): ${addDuration.toFixed(4)} seconds`)
    console.log(`Average time per addition: ${(addDuration * 1000000 / iterations).toFixed(2)} microseconds`)

    console.log(`\nScalar operations (${iterations} iterations): ${scalarDuration.toFixed(4)} seconds`)
    console.log(`Average time per scalar operation: ${(scalarDuration * 1000000 / iterations).toFixed(2)} microseconds`)

    console.log(`\nMultiple operations (${iterations} iterations): ${multiDuration.toFixed(4)} seconds`)
    console.log(`Average time per multiple operation: ${(multiDuration * 1000000 / iterations).toFixed(2)} microseconds`)

    console.log(`\nComplex operations (${iterations} iterations): ${complexDuration.toFixed(4)} seconds`)
    console.log(`Average time per complex operation: ${(complexDuration * 1000000 / iterations).toFixed(2)} microseconds`)

    // Print sample results
    console.log("\nSample results:")
    console.log("First 5 elements of result:",
        result[0], result[1], result[2], result[3], result[4])
}

runBenchmark()
