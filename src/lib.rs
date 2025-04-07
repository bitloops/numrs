#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use napi::{JsNumber, JsObject, JsUnknown, NapiRaw, NapiValue}; // Napi specific types
use ndarray::{Array, ArrayD, IxDyn}; // ndarray types

// Define the main struct that will be exposed to JavaScript
#[napi(js_name = "NdArray")]
pub struct JsNdArray {
    // Internal representation using ndarray
    // We use ArrayD<f64> for dynamic dimensions and f64 data type
    array: ArrayD<f64>,
}

#[napi]
impl JsNdArray {
    // --- Constructor ---
    #[napi(constructor)]
    pub fn new(env: Env, shape_or_data: JsUnknown) -> Result<Self> {
        // Determine if the input is shape (array of numbers) or data (nested array)
        match shape_or_data.get_type()? {
            ValueType::Object => {
                // Assume it's data (nested array)
                if !shape_or_data.is_array()? {
                     return Err(napi::Error::new(
                        Status::InvalidArg,
                        "Input must be an array (for data) or an array of numbers (for shape)."
                            .to_string(),
                    ));
                }
                let js_array = unsafe { JsObject::from_raw_unchecked(env.raw(), shape_or_data.raw()) };
                let (data, shape) = js_array_to_vec_f64(&env, &js_array, 0)?;
                let array = Array::from_shape_vec(IxDyn(&shape), data)
                    .map_err(|e| napi::Error::new(Status::InvalidArg, format!("Failed to create array from shape and data: {}", e)))?;
                Ok(Self { array })
            }
            _ => Err(napi::Error::new(
                Status::InvalidArg,
                "Constructor expects a nested array (data) or an array of numbers (shape - to be implemented).".to_string(),
            )),
        }
    }

    // --- Properties ---
    #[napi(getter)]
    pub fn shape(&self, env: Env) -> Result<Vec<u32>> {
        // ndarray shape is Vec<usize>, convert to Vec<u32> for JS (numbers are typically f64 or i32)
        Ok(self.array.shape().iter().map(|&d| d as u32).collect())
    }

    #[napi(getter)]
    pub fn size(&self) -> Result<u32> {
        Ok(self.array.len() as u32)
    }

    #[napi(getter)]
    pub fn ndim(&self) -> Result<u32> {
        Ok(self.array.ndim() as u32)
    }

    #[napi(getter)]
    pub fn dtype(&self) -> Result<&'static str> {
        // For now, we only support f64
        Ok("float64")
    }

    // --- Methods ---
    #[napi]
    pub fn to_string(&self) -> Result<String> {
        Ok(format!("{}", self.array))
    }

     // Example: Get a single element (more complex slicing/indexing later)
    #[napi]
    pub fn get(&self, indices: Vec<u32>) -> Result<f64> {
        let ix: Vec<usize> = indices.iter().map(|&i| i as usize).collect();
        self.array.get(IxDyn(&ix))
            .map(|v| *v) // Dereference the Option<&f64> to get f64
            .ok_or_else(|| napi::Error::new(Status::InvalidArg, format!("Index {:?} out of bounds for shape {:?}", ix, self.array.shape())))
    }

    // Example: Element-wise addition with another NdArray
    #[napi]
    pub fn add(&self, other: &JsNdArray) -> Result<JsNdArray> {
        // ndarray handles broadcasting automatically if shapes are compatible
        let result_array = &self.array + &other.array; // This might panic on shape mismatch in ndarray 0.15; need error handling
        // In newer ndarray or with specific configurations, it returns Result
        // TODO: Add proper shape compatibility check or use try_add if available
        Ok(JsNdArray { array: result_array })
    }

     // Example: Element-wise addition with a scalar
     #[napi]
     pub fn add_scalar(&self, scalar: f64) -> Result<JsNdArray> {
         let result_array = &self.array + scalar;
         Ok(JsNdArray { array: result_array })
     }
}

// --- Helper Function ---
// Recursive helper to parse nested JS arrays into flat Vec<f64> and determine shape
// This is a simplified version and needs more robust error checking (jagged arrays, non-number types)
fn js_array_to_vec_f64(
    env: &Env,
    js_arr: &JsObject, // Use JsObject as JsArray is just a typed JsObject
    depth: usize,
) -> Result<(Vec<f64>, Vec<usize>)> {
    let len = js_arr.get_array_length()? as usize;
    if len == 0 {
        // Handle empty array case - decide on shape representation (e.g., [0] or [?, 0])
        if depth == 0 {
             return Err(napi::Error::new(Status::InvalidArg,"Input array cannot be empty at the top level.".to_string()));
        } else {
            // Allow empty inner arrays, but shape determination gets tricky.
            // For simplicity here, assume non-empty or fail. A real implementation needs careful handling.
             return Err(napi::Error::new(Status::InvalidArg, format!("Empty array found at depth {}", depth)));
        }
    }

    let mut data: Vec<f64> = Vec::new();
    let mut shape: Vec<usize> = Vec::new();
    let mut first_inner_shape: Option<Vec<usize>> = None;

    for i in 0..len {
        let element: JsUnknown = js_arr.get_element(i as u32)?;
        match element.get_type()? {
            ValueType::Number => {
                // If we encounter numbers, we must be at the deepest level
                 if depth == 0 && shape.is_empty() { // Shape for 1D array
                    shape.push(len);
                } else if !shape.is_empty() && shape.len() != 1 {
                    // Mixing depths - e.g. [1, [2]] - this is a jagged array
                    return Err(napi::Error::new(Status::InvalidArg, format!("Jagged array detected at index {}", i)));
                }
                let num: f64 = JsNumber::try_from(element)?.get_double()?;
                data.push(num);
            }
            ValueType::Object => {
                // If we encounter an object, assume it's a nested array
                if !element.is_array()? {
                     return Err(napi::Error::new(Status::InvalidArg, format!("Non-array object found at index {}", i)));
                }
                 if !shape.is_empty() && shape.len() > 1 {
                     // We already processed deeper arrays, finding another object means jagged array
                     return Err(napi::Error::new(Status::InvalidArg, format!("Jagged array detected (object after numbers) at index {}", i)));
                }

                let inner_js_arr = unsafe { JsObject::from_raw_unchecked(env.raw(), element.raw()) };
                let (inner_data, inner_shape) = js_array_to_vec_f64(env, &inner_js_arr, depth + 1)?;

                if let Some(ref expected_shape) = first_inner_shape {
                    if *expected_shape != inner_shape {
                        return Err(napi::Error::new(Status::InvalidArg, format!("Jagged array detected: shapes mismatch at index {}. Expected {:?}, found {:?}", i, expected_shape, inner_shape)));
                    }
                } else {
                    first_inner_shape = Some(inner_shape.clone());
                    // Determine the full shape on the first inner element
                    shape.push(len); // Current dimension's length
                    shape.extend(inner_shape); // Add inner dimensions
                }
                data.extend(inner_data);
            }
            _ => {
                return Err(napi::Error::new(Status::InvalidArg, format!("Unsupported type found in array at index {}: {:?}", i, element.get_type()?)));
            }
        }
    }

     // If shape is still empty here, it means it was likely a 1D array of numbers handled inside the loop
    if shape.is_empty() && !data.is_empty() && depth == 0 {
        shape.push(data.len());
    } else if shape.is_empty() && data.is_empty() && len > 0 {
         // Array of empty arrays? Needs specific handling. Erroring out for now.
         return Err(napi::Error::new(Status::InvalidArg, "Array contains only empty arrays - shape is ambiguous.".to_string()));
    }


    Ok((data, shape))
}