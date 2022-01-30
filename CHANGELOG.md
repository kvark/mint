# Change Log

### v0.5.9 (XX-XX-2022)
- implement `AsMut<array>`

### v0.5.8 (22-10-2021)
- add `IntoMint` trait for unique conversions ([#68])

[#68]: https://github.com/kvark/mint/pull/68

### v0.5.7 (25-09-2021)
- update to rust2018
- implement `From<mint::Xxx>` for fixed-size arrays

### v0.5.6 (04-11-2020)
- add serde support

### v0.5.5 (13-04-2020)
- fixed the `Quaternion` memory layout

### v0.5.4 (18-11-2019)
- make it `[no_std]`

### v0.5.3 (30-09-2019)
- removed executable flag from the crate files

### v0.5.2 (16-09-2019)
- even more matrices

### v0.5.1 (23-07-2018)
- more asymmetrical matrix types

## v0.5.0 (28-12-2017)
- removed `LeftQuaternion`
- implement `AsRef`

### v0.4.2 (08-06-2017)
- derive traits for rotation types

### v0.4.1 (06-06-2017)
- major refactor of Euler angles

## v0.3.0 (01-06-2017)
- added strongly-typed Row/Column matrices
- added points

## v0.2.0 (29-05-2017)
- removed Row/Column matrix separation
- added `QuatScalePos`

## v0.1.0 (26-05-2017)
- basic matrices, vectors, quaternions, and Euler angles
