#![deny(
    missing_docs,
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    unused,
    clippy::all
)]

use mint::{
    ColumnMatrix2, ColumnMatrix2x3, ColumnMatrix2x4, ColumnMatrix3, ColumnMatrix3x2,
    ColumnMatrix3x4, ColumnMatrix4, ColumnMatrix4x2, ColumnMatrix4x3,
};
use mint::{EulerAngles, Point2, Point3, Quaternion, Vector2, Vector3, Vector4};
use mint::{
    RowMatrix2, RowMatrix2x3, RowMatrix2x4, RowMatrix3, RowMatrix3x2, RowMatrix3x4, RowMatrix4,
    RowMatrix4x2, RowMatrix4x3,
};

macro_rules! transitive {
    ($name:ident [ $($field:ident = $value:expr),* ] = $fixed:ty) => (
        let v1 = $name { $($field : $value,)* };
        let a: $fixed = v1.into();
        let v2 = $name::from(a);
        assert_eq!(v1, v2);
    );

    ($name:ident [ $($field:ident = $value:expr),* ] = ref $fixed:ty) => (
        transitive!($name [ $($field = $value),* ] = $fixed);
        let v1 = $name { $($field : $value,)* };
        let a: $fixed = v1.into();
        assert_eq!(AsRef::<$fixed>::as_ref(&v1), &a);
    );
}

macro_rules! matrix_transitive {
    ($name:ident $vecType:ident[ $($field:ident = $value:expr),* ] = ($inner:expr, $outer:expr) : $elt:ty) => (
        transitive!($name [ $($field = $vecType::from($value)),*] = ref [[$elt; $inner]; $outer]);
        transitive!($name [ $($field = $vecType::from($value)),*] = ref [$elt; $inner * $outer]);
    )
}

macro_rules! turn {
    ($name1:ident [$($value1:expr),*] = $name2:ident [$($value2:expr),*]) => (
        let transposed = [$($value2),*];
        let m1 = $name1::from([$($value1),*]);
        let m2: $name2<_> = m1.into();
        assert_eq!(m1, $name1::from(m2));
        assert_eq!(m2, $name2::from(transposed));
    )
}

#[test]
fn vector() {
    transitive!(Vector2 [x=1, y=3] = ref [i32; 2]);
    transitive!(Vector3 [x=1, y=3, z=5] = ref [i32; 3]);
    transitive!(Vector4 [x=1, y=3, z=5, w=7] = ref [i32; 4]);
    transitive!(Point2 [x=1, y=3] = ref [i32; 2]);
    transitive!(Point3 [x=1, y=3, z=5] = ref [i32; 3]);
    // Translation Vector <-> Point
    transitive!(Point2 [x=1, y=3] = Vector2<i32>);
    transitive!(Point3 [x=1, y=3, z=5] = Vector3<i32>);
}

#[test]
fn rotation() {
    transitive!(Quaternion [s=1, v=Vector3{x: 1, y: 3, z: 5}] = [i32; 4]);
    // EulerAngles
    let a1: [i32; 3] = [1, 3, 5];
    let e: EulerAngles<_, mint::ExtraXYZ> = EulerAngles::from(a1);
    let a2: [i32; 3] = e.into();
    assert_eq!(a1, a2);
}

#[test]
fn row_matrix() {
    matrix_transitive!(RowMatrix2 Vector2[
        x=[1,2],
        y=[3,4]]
        = (2, 2): i32);
    matrix_transitive!(RowMatrix2x3 Vector3[
        x=[1,2,3],
        y=[4,5,6]]
        = (3, 2): i32);
    matrix_transitive!(RowMatrix2x4 Vector4[
        x=[1,2,3,4],
        y=[5,6,7,8]]
        = (4, 2): i32);
    matrix_transitive!(RowMatrix3x2 Vector2[
        x=[1,2],
        y=[3,4],
        z=[5,6]]
        = (2, 3): i32);
    matrix_transitive!(RowMatrix3 Vector3[
        x=[1,2,3],
        y=[4,5,6],
        z=[7,8,9]]
        = (3, 3): i32);
    matrix_transitive!(RowMatrix3x4 Vector4[
        x=[1,2,3,4],
        y=[5,6,7,8],
        z=[9,10,11,12]]
        = (4, 3): i32);
    matrix_transitive!(RowMatrix4x2 Vector2[
        x=[1,2],
        y=[3,4],
        z=[5,6],
        w=[7,8]]
        = (2, 4): i32);
    matrix_transitive!(RowMatrix4x3 Vector3[
        x=[1,2,3],
        y=[4,5,6],
        z=[7,8,9],
        w=[10,11,12]]
        = (3, 4): i32);
    matrix_transitive!(RowMatrix4 Vector4[
        x=[1,2,3,4],
        y=[5,6,7,8],
        z=[9,10,11,12],
        w=[13,14,15,16]]
        = (4, 4): i32);
}

#[test]
fn column_matrix() {
    matrix_transitive!(ColumnMatrix2 Vector2[
        x=[1,2],
        y=[3,4]]
        = (2, 2): i32);
    matrix_transitive!(ColumnMatrix2x3 Vector2[
        x=[1,2],
        y=[3,4],
        z=[5,6]]
        = (2, 3): i32);
    matrix_transitive!(ColumnMatrix2x4 Vector2[
        x=[1,2],
        y=[3,4],
        z=[5,6],
        w=[7,8]]
        = (2, 4): i32);
    matrix_transitive!(ColumnMatrix3x2 Vector3[
        x=[1,2,3],
        y=[4,5,6]]
        = (3, 2): i32);
    matrix_transitive!(ColumnMatrix3 Vector3[
        x=[1,2,3],
        y=[4,5,6],
        z=[7,8,9]]
        = (3, 3): i32);
    matrix_transitive!(ColumnMatrix3x4 Vector3[
        x=[1,2,3],
        y=[4,5,6],
        z=[7,8,9],
        w=[10,11,12]]
        = (3, 4): i32);
    matrix_transitive!(ColumnMatrix4x2 Vector4[
        x=[1,2,3,4],
        y=[5,6,7,8]]
        = (4, 2): i32);
    matrix_transitive!(ColumnMatrix4x3 Vector4[
        x=[1,2,3,4],
        y=[5,6,7,8],
        z=[9,10,11,12]]
        = (4, 3): i32);
    matrix_transitive!(ColumnMatrix4 Vector4[
        x=[1,2,3,4],
        y=[5,6,7,8],
        z=[9,10,11,12],
        w=[13,14,15,16]]
        = (4, 4): i32);
}

#[test]
fn turn() {
    turn!(RowMatrix2 [
        [1,2],
        [3,4]]
        = ColumnMatrix2 [
        [1,3],
        [2,4]]);
    turn!(RowMatrix2x3 [
        [1,3,5],
        [2,4,6]]
        = ColumnMatrix2x3 [
        [1,2],
        [3,4],
        [5,6]]);
    turn!(RowMatrix2x4 [
        [1,3,5,7],
        [2,4,6,8]]
        = ColumnMatrix2x4 [
        [1,2],
        [3,4],
        [5,6],
        [7,8]]);
    turn!(RowMatrix3x2 [
        [1,2],
        [3,4],
        [5,6]]
        = ColumnMatrix3x2 [
        [1,3,5],
        [2,4,6]]);
    turn!(RowMatrix3 [
        [1,2,3],
        [4,5,6],
        [7,8,9]]
        = ColumnMatrix3 [
        [1,4,7],
        [2,5,8],
        [3,6,9]]);
    turn!(RowMatrix3x4 [
        [1,4,7,10],
        [2,5,8,11],
        [3,6,9,12]]
        = ColumnMatrix3x4 [
        [1,2,3],
        [4,5,6],
        [7,8,9],
        [10,11,12]]);
    turn!(RowMatrix4x2 [
        [1,2],
        [3,4],
        [5,6],
        [7,8]]
        = ColumnMatrix4x2 [
        [1,3,5,7],
        [2,4,6,8]]);
    turn!(RowMatrix4x3 [
        [1,2,3],
        [4,5,6],
        [7,8,9],
        [10,11,12]]
        = ColumnMatrix4x3 [
        [1,4,7,10],
        [2,5,8,11],
        [3,6,9,12]]);
    turn!(RowMatrix4 [
        [1,2,3,4],
        [5,6,7,8],
        [9,10,11,12],
        [13,14,15,16]]
        = ColumnMatrix4 [
        [1,5,9,13],
        [2,6,10,14],
        [3,7,11,15],
        [4,8,12,16]]);
}

#[test]
fn vector_from_slice_success() {
    let v = Vector3::from_slice(&[0.0f32, 1.0, 2.0, 3.0]);
    assert_eq!(
        v,
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 2.0
        }
    );
}

#[test]
#[should_panic(expected = "Missing y-axis in slice.")]
fn vector_from_slice_fail() {
    let _ = Vector4::from_slice(&[0.0]);
}

#[test]
fn quaternion_layout() {
    let q = Quaternion {
        v: Vector3::from([0, 1, 2]),
        s: 3,
    };
    let expected = [0, 1, 2, 3];

    let a: [i32; 4] = q.into();
    assert_eq!(a, expected);

    let b: &[i32; 4] = q.as_ref();
    assert_eq!(b, &expected);

    let c: [i32; 4] = unsafe { core::mem::transmute(q) };
    assert_eq!(c, expected);
}

macro_rules! representation_tests {
    ($module:ident => $name:ty : $fixed:ty ) => {
        #[cfg(all(feature = "serde", test))]
        mod $module {
            use super::*;
            use serde_json;

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
    };
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

representation_tests!( euler_angles => EulerAngles<f32, f32> : [f32; 3] );
representation_tests!( quaternions => Quaternion<f32> : [f32; 4] );
