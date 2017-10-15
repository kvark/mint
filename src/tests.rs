
macro_rules! representation_tests {
    ($module:ident => $name:ty : $fixed:ty ) => {
        #[cfg(all(feature = "serde", test))]
        mod $module {
            extern crate serde_json;
            use ::*;

            #[test]
            fn serialize() {
                let fixed: $fixed = <$fixed as Default>::default();
                let fixed_valid = serde_json::to_string(&fixed).unwrap();

                // Should do the same things as the $fixed serialized representation.
                let this: $name = fixed.into();
                let this_valid = serde_json::to_string(&this).unwrap();

                assert_eq!(fixed_valid, this_valid);
            }

            #[test]
            fn deserialize() {
                // Should be able to create $name from the $fixed representation.
                let fixed: $fixed = <$fixed as Default>::default();
                let fixed_valid = serde_json::to_string(&fixed).unwrap();

                let from_string: $name = serde_json::from_str(&fixed_valid).unwrap();
                let from_fixed: $name = fixed.into();

                // Serializing the struct again should be the same as the $fixed representation.
                let into_string = serde_json::to_string(&from_string).unwrap();

                assert_eq!(from_string, from_fixed);
                assert_eq!(into_string, fixed_valid);
            }
        }
    }
}

representation_tests!( row_matrix_2_serde => RowMatrix2<f32> : [[f32; 2]; 2] );
representation_tests!( row_matrix_2x3_serde => RowMatrix2x3<f32> : [[f32; 3]; 2] );
representation_tests!( row_matrix_3_serde => RowMatrix3<f32> : [[f32; 3]; 3] );
representation_tests!( row_matrix_3x4_serde => RowMatrix3x4<f32> : [[f32; 4]; 3] );
representation_tests!( row_matrix_4_serde => RowMatrix4<f32> : [[f32; 4]; 4] );

representation_tests!( column_matrix_2_serde => ColumnMatrix2<f32> : [[f32; 2]; 2] );
representation_tests!( column_matrix_2x3_serde => ColumnMatrix2x3<f32> : [[f32; 2]; 3] );
representation_tests!( column_matrix_3_serde => ColumnMatrix3<f32> : [[f32; 3]; 3] );
representation_tests!( column_matrix_3x4_serde => ColumnMatrix3x4<f32> : [[f32; 3]; 4] );
representation_tests!( column_matrix_4_serde => ColumnMatrix4<f32> : [[f32; 4]; 4] );

representation_tests!( vector2_serde => Vector2<f32> : [f32; 2] );
representation_tests!( vector3_serde => Vector3<f32> : [f32; 3] );
representation_tests!( vector4_serde => Vector4<f32> : [f32; 4] );
representation_tests!( point2_serde => Point2<f32> : [f32; 2] );
representation_tests!( point3_serde => Point3<f32> : [f32; 3] );
