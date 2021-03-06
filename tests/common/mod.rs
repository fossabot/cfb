#![macro_use]

#[rustfmt::skip]
pub mod ckb_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod ckb_generated;
#[rustfmt::skip]
pub mod ckb_generated_verifier;
#[rustfmt::skip]
pub mod data_alignment_builder;
#[rustfmt::skip]
pub mod data_order_builder;
#[rustfmt::skip]
pub mod enum_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod enum_generated;
#[rustfmt::skip]
pub mod enum_vector_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod enum_vector_generated;
#[rustfmt::skip]
pub mod nested_buffer_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod nested_buffer_generated;
#[rustfmt::skip]
pub mod scalar_vector_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod scalar_vector_generated;
#[rustfmt::skip]
pub mod scalar_vector_generated_verifier;
#[rustfmt::skip]
pub mod scalars_with_different_size_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod scalars_with_different_size_generated;
#[rustfmt::skip]
pub mod scalars_with_same_size_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod scalars_with_same_size_generated;
#[rustfmt::skip]
pub mod string_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod string_generated;
#[rustfmt::skip]
pub mod string_generated_verifier;
#[rustfmt::skip]
pub mod string_vector_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod string_vector_generated;
#[rustfmt::skip]
pub mod string_vector_generated_verifier;
#[rustfmt::skip]
pub mod struct_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod struct_generated;
#[rustfmt::skip]
pub mod struct_vector_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod struct_vector_generated;
#[rustfmt::skip]
pub mod table_field_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod table_field_generated;
#[rustfmt::skip]
pub mod table_field_generated_verifier;
#[rustfmt::skip]
pub mod table_fields_order_builder;
#[rustfmt::skip]
pub mod table_vector_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod table_vector_generated;
#[rustfmt::skip]
pub mod table_vector_generated_verifier;
#[rustfmt::skip]
pub mod union_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod union_generated;
#[rustfmt::skip]
pub mod union_generated_verifier;

use flatbuffers::{Follow, Vector};

#[macro_export]
macro_rules! le {
    ($e:expr) => {
        &(($e).to_le_bytes())[..]
    };
}

pub fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join("")
}

pub fn from_hex(hex: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}

pub fn collect_flatbuffers_vector<'a, T: Follow<'a> + 'a>(vec: &Vector<'a, T>) -> Vec<T::Inner> {
    let mut collected = Vec::with_capacity(vec.len());

    for i in 0..vec.len() {
        collected.push(vec.get(i))
    }

    collected
}
