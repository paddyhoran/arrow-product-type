use crate::metadata::MetaData;

use arrow_array::{PrimitiveArray, types::Float64Type};

/// This is the main type used to model data of varying
/// granularity.
pub struct Data {
    /// Holds the meta-data so we know how to interpret the
    /// `values`.
    metadata: MetaData,

    /// Holds the actual values.
    values: PrimitiveArray<Float64Type>,
}

impl Data {
    /// Creates a new piece of data that contains a single dimension.
    pub fn new(dimension_name: String, dimension_values: Vec<String>, values: Vec<f64>) -> Self {
        let metadata = MetaData::new(dimension_name, dimension_values);
        let values = PrimitiveArray::<Float64Type>::from(values);

        Self { metadata, values }
    }

    /// Creates a new piece of data that contains a single dimension from an iterator.
    pub fn new_from_iter(
        dimension_name: String,
        iter: impl Iterator<Item = (String, f64)>,
    ) -> Self {
        let (dimension_values, values) = iter.unzip();
        Self::new(dimension_name, dimension_values, values)
    }
}
