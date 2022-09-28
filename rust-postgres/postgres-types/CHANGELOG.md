# Change Log

## v0.2.3 - 2022-04-30

### Added

* Added `ToSql` and `FromSql` implementations for `Box<str>`.
* Added `BorrowToSql` implementations for `Box<dyn ToSql + Sync>` and `Box<dyn ToSql + Sync + Send>`.
* Added support for `cidr` 0.2 via the `with-cidr-02` feature.
* Added conversions between the `LTREE`, `LQUERY` and `LTXTQUERY` types and Rust strings.
* Added support for `uuid` 1.0 via the `with-uuid-1` feature.

## v0.2.2 - 2021-09-29

### Added

* Added support for `eui48` 1.0 via the `with-eui48-1` feature.
* Added `ToSql` and `FromSql` implementations for array types via the `array-impls` feature.
* Added support for `time` 0.3 via the `with-time-0_3` feature.

## v0.2.1 - 2021-04-03

### Added

* Added support for `geo-types` 0.7 via `with-geo-types-0_7` feature.
* Added the `PgLsn` type, corresponding to `PG_LSN`.

## v0.2.0 - 2020-12-25

### Changed

* Upgraded `bytes` to 1.0.

### Removed

* Removed support for `geo-types` 0.4.

## v0.1.3 - 2020-10-17

### Added

* Implemented `Clone`, `PartialEq`, and `Eq` for `Json`.

### Fixed

* Checked for overflow in `NaiveDate` and `NaiveDateTime` conversions.

## v0.1.2 - 2020-07-03

### Added

* Added support for `geo-types` 0.6.

## v0.1.1 - 2020-03-05

### Added

* Added support for `time` 0.2.

## v0.1.0 - 2019-12-23

### Changed

* `Kind` is now a true non-exhaustive enum.

### Removed

* Removed `uuid` 0.7 support.

### Added

* Added a `Hash` implementation for `Type`.

## v0.1.0-alpha.2 - 2019-11-27

### Changed

* Upgraded `bytes` to 0.5.
* Upgraded `uuid` to 0.8.

## v0.1.0-alpha.1 - 2019-10-14

Initial release
