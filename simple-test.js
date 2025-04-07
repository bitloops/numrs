const { NdArray } = require('./index')

// Create two 1D arrays
const arr1 = new NdArray([1, 2, 3])
const arr2 = new NdArray([4, 5, 6])

// Test array properties
console.log('Array 1 shape:', arr1.shape)
console.log('Array 1 size:', arr1.size)
console.log('Array 1 ndim:', arr1.ndim)
console.log('Array 1 dtype:', arr1.dtype)

// Test element access
console.log('Element at index 1:', arr1.get([1]))

// Test addition between arrays
const sumArray = arr1.add(arr2)
console.log('Sum of arrays:', sumArray.toString())

// Test scalar addition
const scalarSum = arr1.addScalar(10)
console.log('Array + scalar:', scalarSum.toString())

// Try to create a 2D array - let's see what happens
try {
  console.log('Attempting to create a 2D array...')
  const arr2d = new NdArray([[1, 2], [3, 4]])
  console.log('2D Array shape:', arr2d.shape)
  console.log('2D Array:', arr2d.toString())
} catch (error) {
  console.log('Error creating 2D array:', error.message)
  console.log('This might be a limitation in the current implementation.')
}

console.log('All tests completed!')
