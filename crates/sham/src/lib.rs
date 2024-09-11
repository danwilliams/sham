//! Sham library.
//! 
//! This library provides mock implementations of various external dependencies
//! for use in testing.
//! 



//		Global configuration

//	Customisations of the standard linting configuration
#![allow(clippy::multiple_crate_versions, reason = "Cannot resolve all these")]
#![allow(clippy::items_after_test_module, reason = "Not needed with separated tests")]

//	Lints specifically disabled for unit tests
#![cfg_attr(test, allow(
	non_snake_case,
	unreachable_pub,
	clippy::cast_lossless,
	clippy::cast_precision_loss,
	clippy::cognitive_complexity,
	clippy::default_numeric_fallback,
	clippy::exhaustive_enums,
	clippy::exhaustive_structs,
	clippy::expect_used,
	clippy::indexing_slicing,
	clippy::let_underscore_must_use,
	clippy::let_underscore_untyped,
	clippy::missing_assert_message,
	clippy::missing_panics_doc,
	clippy::must_use_candidate,
	clippy::panic,
	clippy::print_stdout,
	clippy::unwrap_in_result,
	clippy::unwrap_used,
	reason = "Not useful in unit tests"
))]


