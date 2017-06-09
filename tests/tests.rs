extern crate mint;
use mint::{
    Vector2, Point2,
    Vector3, Point3,
    Vector4, Quaternion,
    LeftQuaternion, EulerAngles};
use mint::{
    RowMatrix2, RowMatrix2x3,
    RowMatrix3, RowMatrix3x4, RowMatrix4};
use mint::{
    ColumnMatrix2, ColumnMatrix3x2,
    ColumnMatrix3, ColumnMatrix4x3, ColumnMatrix4};

macro_rules! transitive {
    ($name:ident [ $($field:ident = $value:expr),* ] = $fixed:ty) => (
        let v1 = $name { $($field : $value,)* };
        let a: $fixed = v1.into();
        let v2 = $name::from(a);
        assert_eq!(v1, v2);
    )
}

macro_rules! matrix_transitive {
    ($name:ident $vecType:ident[ $($field:ident = $value:expr),* ] = $fixed:ty) => (
        transitive!($name [ $($field = $vecType::from($value)),*] = $fixed);
    )
}

macro_rules! transpose {
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
    transitive!(Vector2 [x=1, y=3] = [i32; 2]);
    transitive!(Vector3 [x=1, y=3, z=5] = [i32; 3]);
    transitive!(Vector4 [x=1, y=3, z=5, w=7] = [i32; 4]);
    transitive!(Point2 [x=1, y=3] = [i32; 2]);
    transitive!(Point3 [x=1, y=3, z=5] = [i32; 3]);
    // Translation Vector <-> Point
    transitive!(Point2 [x=1, y=3] = Vector2<i32>);
    transitive!(Point3 [x=1, y=3, z=5] = Vector3<i32>);
}

#[test]
fn rotation() {
    transitive!(Quaternion [s=1, v=Vector3{x: 1, y: 3, z: 5}] = [i32; 4]);
    // LeftQuaternion
    let a1: [i32; 4] = [1, 3, 5, 7];
    let q: LeftQuaternion<_, mint::MirrorX> = LeftQuaternion::from(a1);
    let a2: [i32; 4] = q.into();
    assert_eq!(a1, a2);
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
        = [[i32; 2]; 2]);
    matrix_transitive!(RowMatrix2x3 Vector2[
        x=[1,2],
        y=[3,4],
        z=[5,6]]
        = [[i32; 2]; 3]);
    matrix_transitive!(RowMatrix3 Vector3[
        x=[1,2,3],
        y=[4,5,6],
        z=[7,8,9]]
        = [[i32; 3]; 3]);
    matrix_transitive!(RowMatrix3x4 Vector3[
        x=[1,2,3],
        y=[4,5,6],
        z=[7,8,9],
        w=[10,11,12]]
        = [[i32; 3]; 4]);
    matrix_transitive!(RowMatrix4 Vector4[
        x=[1,2,3,4],
        y=[5,6,7,8],
        z=[9,10,11,12],
        w=[13,14,15,16]]
        = [[i32; 4]; 4]);
}

#[test]
fn column_matrix() {
    matrix_transitive!(ColumnMatrix2 Vector2[
        x=[1,2],
        y=[3,4]]
        = [[i32; 2]; 2]);
    matrix_transitive!(ColumnMatrix3x2 Vector3[
        x=[1,2,3],
        y=[4,5,6]]
        = [[i32; 3]; 2]);
    matrix_transitive!(ColumnMatrix3 Vector3[
        x=[1,2,3],
        y=[4,5,6],
        z=[7,8,9]]
        = [[i32; 3]; 3]);
    matrix_transitive!(ColumnMatrix4x3 Vector4[
        x=[1,2,3,4],
        y=[5,6,7,8],
        z=[9,10,11,12]]
        = [[i32; 4]; 3]);
    matrix_transitive!(ColumnMatrix4 Vector4[
        x=[1,2,3,4],
        y=[5,6,7,8],
        z=[9,10,11,12],
        w=[13,14,15,16]]
        = [[i32; 4]; 4]);
}

#[test]
fn transpose() {
    transpose!(RowMatrix2 [
        [1,2],
        [3,4]]
        = ColumnMatrix2 [
        [1,3],
        [2,4]]);
    transpose!(RowMatrix2x3 [
        [1,2],
        [3,4],
        [5,6]]
        = ColumnMatrix3x2 [
        [1,3,5],
        [2,4,6]]);
    transpose!(RowMatrix3 [
        [1,2,3],
        [4,5,6],
        [7,8,9]]
        = ColumnMatrix3 [
        [1,4,7],
        [2,5,8],
        [3,6,9]]);
    transpose!(RowMatrix3x4 [
        [1,2,3],
        [4,5,6],
        [7,8,9],
        [10,11,12]]
        = ColumnMatrix4x3 [
        [1,4,7,10],
        [2,5,8,11],
        [3,6,9,12]]);
    transpose!(RowMatrix4 [
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
