#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use napi::{JsNumber, JsObject, JsUnknown, NapiRaw, NapiValue};
use ndarray::{Array, ArrayD, IxDyn};
use std::sync::Arc;

// Define the main struct that will be exposed to JavaScript
#[napi(js_name = "NdArray")]
#[derive(Clone)]
pub struct JsNdArray {
    array: Arc<ArrayD<f64>>, // Use Arc for cheap cloning and memory reuse
}

// Internal helper struct for batch operations
#[derive(Default)]
struct BatchOperations {
    shape: Vec<usize>,
    data: Vec<f64>,
}

#[napi]
impl JsNdArray {
    #[inline(always)]
    #[napi(constructor)]
    pub fn new(env: Env, shape_or_data: JsUnknown) -> Result<Self> {
        match shape_or_data.get_type()? {
            ValueType::Object => {
                if !shape_or_data.is_array()? {
                    return Err(Error::new(
                        Status::InvalidArg,
                        "Input must be an array".to_string(),
                    ));
                }
                let js_array = unsafe { JsObject::from_raw_unchecked(env.raw(), shape_or_data.raw()) };
                let (data, shape) = js_array_to_vec_f64(&env, &js_array, 0)?;
                let array = Array::from_shape_vec(IxDyn(&shape), data)
                    .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
                Ok(Self { array: Arc::new(array) })
            }
            _ => Err(Error::new(
                Status::InvalidArg,
                "Constructor expects an array".to_string(),
            )),
        }
    }

    // --- Properties with inline optimization ---
    #[inline(always)]
    #[napi(getter)]
    pub fn shape(&self) -> Result<Vec<u32>> {
        Ok(self.array.shape().iter().map(|&d| d as u32).collect())
    }

    #[inline(always)]
    #[napi(getter)]
    pub fn size(&self) -> Result<u32> {
        Ok(self.array.len() as u32)
    }

    #[inline(always)]
    #[napi(getter)]
    pub fn ndim(&self) -> Result<u32> {
        Ok(self.array.ndim() as u32)
    }

    #[inline(always)]
    #[napi(getter)]
    pub fn dtype(&self) -> Result<&'static str> {
        Ok("float64")
    }

    #[inline]
    #[napi]
    pub fn get(&self, indices: Vec<u32>) -> Result<f64> {
        let ix: Vec<usize> = indices.iter().map(|&i| i as usize).collect();
        self.array
            .get(IxDyn(&ix))
            .copied()
            .ok_or_else(|| Error::new(Status::InvalidArg, "Index out of bounds".to_string()))
    }

    // Optimized addition using chunks
    #[inline]
    #[napi]
    pub fn add(&self, other: &JsNdArray) -> Result<JsNdArray> {
        if self.array.shape() != other.array.shape() {
            return Err(Error::new(Status::InvalidArg, "Shape mismatch".to_string()));
        }

        let mut result_data = Vec::with_capacity(self.array.len());
        let self_slice = self.array.as_slice().unwrap();
        let other_slice = other.array.as_slice().unwrap();

        // Process elements in chunks for better cache utilization
        for (a, b) in self_slice.chunks(64).zip(other_slice.chunks(64)) {
            result_data.extend(a.iter().zip(b.iter()).map(|(&x, &y)| x + y));
        }

        let array = Array::from_shape_vec(IxDyn(self.array.shape()), result_data)
            .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;

        Ok(JsNdArray { array: Arc::new(array) })
    }

    // Optimized scalar addition using chunks
    #[inline]
    #[napi]
    pub fn add_scalar(&self, scalar: f64) -> Result<JsNdArray> {
        let mut result_data = Vec::with_capacity(self.array.len());
        let self_slice = self.array.as_slice().unwrap();

        // Process elements in chunks for better cache utilization
        for chunk in self_slice.chunks(64) {
            result_data.extend(chunk.iter().map(|&x| x + scalar));
        }

        let array = Array::from_shape_vec(IxDyn(self.array.shape()), result_data)
            .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;

        Ok(JsNdArray { array: Arc::new(array) })
    }

    // Method chaining support
    #[napi]
    pub fn chain(&self) -> Result<JsNdArray> {
        Ok(self.clone())
    }
}

// Optimized helper function
#[inline]
fn js_array_to_vec_f64(
    env: &Env,
    js_arr: &JsObject,
    depth: usize,
) -> Result<(Vec<f64>, Vec<usize>)> {
    let len = js_arr.get_array_length()? as usize;
    if len == 0 {
        return Err(Error::new(Status::InvalidArg, "Empty array not supported".to_string()));
    }

    let mut data = Vec::with_capacity(len);
    let mut shape = Vec::with_capacity(2); // Most common case: 1D or 2D arrays
    let mut first_inner_shape = None;

    for i in 0..len {
        let element: JsUnknown = js_arr.get_element(i as u32)?;
        match element.get_type()? {
            ValueType::Number => {
                if depth == 0 && shape.is_empty() {
                    shape.push(len);
                }
                let num = JsNumber::try_from(element)?.get_double()?;
                data.push(num);
            }
            ValueType::Object => {
                if !element.is_array()? {
                    return Err(Error::new(Status::InvalidArg, "Non-array object found".to_string()));
                }
                let inner_js_arr = unsafe { JsObject::from_raw_unchecked(env.raw(), element.raw()) };
                let (inner_data, inner_shape) = js_array_to_vec_f64(env, &inner_js_arr, depth + 1)?;

                if let Some(ref expected_shape) = first_inner_shape {
                    if *expected_shape != inner_shape {
                        return Err(Error::new(Status::InvalidArg, "Jagged array detected".to_string()));
                    }
                } else {
                    first_inner_shape = Some(inner_shape.clone());
                    shape.push(len);
                    shape.extend(inner_shape);
                }
                data.extend(inner_data);
            }
            _ => return Err(Error::new(Status::InvalidArg, "Unsupported type".to_string())),
        }
    }

    if shape.is_empty() && !data.is_empty() && depth == 0 {
        shape.push(data.len());
    }

    Ok((data, shape))
}
