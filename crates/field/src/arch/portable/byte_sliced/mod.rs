// Copyright 2024-2025 Irreducible Inc.

mod invert;
mod multiply;
mod packed_byte_sliced;
mod square;

pub use packed_byte_sliced::*;

#[cfg(test)]
pub mod tests {
	macro_rules! define_byte_sliced_test {
		($module_name:ident, $name:ident, $scalar_type:ty) => {
			mod $module_name{
				use proptest::prelude::*;
				use crate::{$scalar_type, underlier::WithUnderlier, packed::PackedField, arch::byte_sliced::$name};

				fn scalar_array_strategy() -> impl Strategy<Value = [$scalar_type; <$name>::WIDTH]> {
					any::<[<$scalar_type as WithUnderlier>::Underlier; <$name>::WIDTH]>().prop_map(|arr| arr.map(<$scalar_type>::from_underlier))
				}

				proptest! {
					#[test]
					fn check_add(scalar_elems_a in scalar_array_strategy(), scalar_elems_b in scalar_array_strategy()) {
						let bytesliced_a = <$name>::from_scalars(scalar_elems_a);
						let bytesliced_b = <$name>::from_scalars(scalar_elems_b);

						let bytesliced_result = bytesliced_a + bytesliced_b;

						for i in 0..<$name>::WIDTH {
							assert_eq!(scalar_elems_a[i] + scalar_elems_b[i], bytesliced_result.get(i));
						}
					}

					#[test]
					fn check_add_assign(scalar_elems_a in scalar_array_strategy(), scalar_elems_b in scalar_array_strategy()) {
						let mut bytesliced_a = <$name>::from_scalars(scalar_elems_a);
						let bytesliced_b = <$name>::from_scalars(scalar_elems_b);

						bytesliced_a += bytesliced_b;

						for i in 0..<$name>::WIDTH {
							assert_eq!(scalar_elems_a[i] + scalar_elems_b[i], bytesliced_a.get(i));
						}
					}

					#[test]
					fn check_sub(scalar_elems_a in scalar_array_strategy(), scalar_elems_b in scalar_array_strategy()) {
						let bytesliced_a = <$name>::from_scalars(scalar_elems_a);
						let bytesliced_b = <$name>::from_scalars(scalar_elems_b);

						let bytesliced_result = bytesliced_a - bytesliced_b;

						for i in 0..<$name>::WIDTH {
							assert_eq!(scalar_elems_a[i] - scalar_elems_b[i], bytesliced_result.get(i));
						}
					}

					#[test]
					fn check_sub_assign(scalar_elems_a in scalar_array_strategy(), scalar_elems_b in scalar_array_strategy()) {
						let mut bytesliced_a = <$name>::from_scalars(scalar_elems_a);
						let bytesliced_b = <$name>::from_scalars(scalar_elems_b);

						bytesliced_a -= bytesliced_b;

						for i in 0..<$name>::WIDTH {
							assert_eq!(scalar_elems_a[i] - scalar_elems_b[i], bytesliced_a.get(i));
						}
					}

					#[test]
					fn check_mul(scalar_elems_a in scalar_array_strategy(), scalar_elems_b in scalar_array_strategy()) {
						let bytesliced_a = <$name>::from_scalars(scalar_elems_a);
						let bytesliced_b = <$name>::from_scalars(scalar_elems_b);

						let bytesliced_result = bytesliced_a * bytesliced_b;

						for i in 0..<$name>::WIDTH {
							assert_eq!(scalar_elems_a[i] * scalar_elems_b[i], bytesliced_result.get(i));
						}
					}

					#[test]
					fn check_mul_assign(scalar_elems_a in scalar_array_strategy(), scalar_elems_b in scalar_array_strategy()) {
						let mut bytesliced_a = <$name>::from_scalars(scalar_elems_a);
						let bytesliced_b = <$name>::from_scalars(scalar_elems_b);

						bytesliced_a *= bytesliced_b;

						for i in 0..<$name>::WIDTH {
							assert_eq!(scalar_elems_a[i] * scalar_elems_b[i], bytesliced_a.get(i));
						}
					}

					#[test]
					fn check_inv(scalar_elems in scalar_array_strategy()) {
						let bytesliced = <$name>::from_scalars(scalar_elems);

						let bytesliced_result = bytesliced.invert_or_zero();

						for (i, scalar_elem) in scalar_elems.iter().enumerate() {
							assert_eq!(scalar_elem.invert_or_zero(), bytesliced_result.get(i));
						}
					}

					#[test]
					fn check_square(scalar_elems in scalar_array_strategy()) {
						let bytesliced = <$name>::from_scalars(scalar_elems);

						let bytesliced_result = bytesliced.square();

						for (i, scalar_elem) in scalar_elems.iter().enumerate() {
							assert_eq!(scalar_elem.square(), bytesliced_result.get(i));
						}
					}

					#[test]
					fn check_linear_transformation(scalar_elems in scalar_array_strategy()) {
						use crate::linear_transformation::{PackedTransformationFactory, FieldLinearTransformation, Transformation};
						use rand::{rngs::StdRng, SeedableRng};

						let bytesliced = <$name>::from_scalars(scalar_elems);

						let linear_transformation = FieldLinearTransformation::random(StdRng::seed_from_u64(0));
						let packed_transformation = <$name>::make_packed_transformation(linear_transformation.clone());

						let bytesliced_result = packed_transformation.transform(&bytesliced);

						for i in 0..<$name>::WIDTH {
							assert_eq!(linear_transformation.transform(&scalar_elems[i]), bytesliced_result.get(i));
						}
					}

					#[test]
					fn check_interleave(scalar_elems_a in scalar_array_strategy(), scalar_elems_b in scalar_array_strategy()) {
						let bytesliced_a = <$name>::from_scalars(scalar_elems_a);
						let bytesliced_b = <$name>::from_scalars(scalar_elems_b);

						for log_block_len in 0..<$name>::LOG_WIDTH {
							let (bytesliced_c, bytesliced_d) = bytesliced_a.interleave(bytesliced_b, log_block_len);

							let block_len = 1 << log_block_len;
							for offset in (0..<$name>::WIDTH).step_by(2 * block_len) {
								for i in 0..block_len {
									assert_eq!(bytesliced_c.get(offset + i), scalar_elems_a[offset + i]);
									assert_eq!(bytesliced_c.get(offset + block_len + i), scalar_elems_b[offset + i]);

									assert_eq!(bytesliced_d.get(offset + i), scalar_elems_a[offset + block_len + i]);
									assert_eq!(bytesliced_d.get(offset + block_len + i), scalar_elems_b[offset + block_len + i]);
								}
							}
						}
					}
				}
			}
		};
	}

	// 128-bit byte-sliced
	define_byte_sliced_test!(tests_16x128, ByteSlicedAES16x128b, AESTowerField128b);
	define_byte_sliced_test!(tests_16x64, ByteSlicedAES16x64b, AESTowerField64b);
	define_byte_sliced_test!(tests_16x32, ByteSlicedAES16x32b, AESTowerField32b);
	define_byte_sliced_test!(tests_16x16, ByteSlicedAES16x16b, AESTowerField16b);
	define_byte_sliced_test!(tests_16x8, ByteSlicedAES16x8b, AESTowerField8b);

	define_byte_sliced_test!(tests_16x16x8b, ByteSlicedAES16x16x8b, AESTowerField8b);
	define_byte_sliced_test!(tests_8x16x8b, ByteSlicedAES8x16x8b, AESTowerField8b);
	define_byte_sliced_test!(tests_4x16x8b, ByteSlicedAES4x16x8b, AESTowerField8b);
	define_byte_sliced_test!(tests_2x16x8b, ByteSlicedAES2x16x8b, AESTowerField8b);

	// 256-bit byte-sliced
	define_byte_sliced_test!(tests_32x128, ByteSlicedAES32x128b, AESTowerField128b);
	define_byte_sliced_test!(tests_32x64, ByteSlicedAES32x64b, AESTowerField64b);
	define_byte_sliced_test!(tests_32x32, ByteSlicedAES32x32b, AESTowerField32b);
	define_byte_sliced_test!(tests_32x16, ByteSlicedAES32x16b, AESTowerField16b);
	define_byte_sliced_test!(tests_32x8, ByteSlicedAES32x8b, AESTowerField8b);

	define_byte_sliced_test!(tests_16x32x8b, ByteSlicedAES16x32x8b, AESTowerField8b);
	define_byte_sliced_test!(tests_8x32x8b, ByteSlicedAES8x32x8b, AESTowerField8b);
	define_byte_sliced_test!(tests_4x32x8b, ByteSlicedAES4x32x8b, AESTowerField8b);
	define_byte_sliced_test!(tests_2x32x8b, ByteSlicedAES2x32x8b, AESTowerField8b);

	// 512-bit byte-sliced
	define_byte_sliced_test!(tests_64x128, ByteSlicedAES64x128b, AESTowerField128b);
	define_byte_sliced_test!(tests_64x64, ByteSlicedAES64x64b, AESTowerField64b);
	define_byte_sliced_test!(tests_64x32, ByteSlicedAES64x32b, AESTowerField32b);
	define_byte_sliced_test!(tests_64x16, ByteSlicedAES64x16b, AESTowerField16b);
	define_byte_sliced_test!(tests_64x8, ByteSlicedAES64x8b, AESTowerField8b);

	define_byte_sliced_test!(tests_16x64x8b, ByteSlicedAES16x64x8b, AESTowerField8b);
	define_byte_sliced_test!(tests_8x64x8b, ByteSlicedAES8x64x8b, AESTowerField8b);
	define_byte_sliced_test!(tests_4x64x8b, ByteSlicedAES4x64x8b, AESTowerField8b);
	define_byte_sliced_test!(tests_2x64x8b, ByteSlicedAES2x64x8b, AESTowerField8b);
}
