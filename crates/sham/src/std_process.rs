//! This module mocks [`std::process`] in order to test commands.
//! 
//! This module mocks the critical parts of [`Command`](std::process::Command)
//! using [`mockall`](https://crates.io/crates/mockall), in order to conduct
//! tests without actually running any system commands. This is important
//! because unit tests should not actually run CLI commands.
//! 
//! Because the [`Command`](std::process::Command) functions use a chaining
//! pattern, there is also a [`FakeCommand`] struct that is used as a wrapper.
//! This sets up the actual mock along with the expected sequence of calls, and
//! then interacts with the mocked functions, whilst returning itself for
//! chaining.
//! 
//! Notably, the mock definitions are more restrictive than the "real" code. If
//! the standard library accepts group X, which includes types A, B, and C, the
//! mocks are reducing the acceptance to just type B only, the type actually
//! used by the application code. This is still compatible with the "real" code.
//! 
//! The approach taken is that the "real" code imports the [`Command`](std::process::Command)
//! from [`std::process`] when running in non-test mode, but imports
//! [`FakeCommand`] when running in test mode. This is achieved by using
//! conditional compilation. The test code then configures the mocks to expect
//! certain requests and to return certain responses, and then runs the tests.
//! 



//		Packages																										

use std::{
	env::args,
	ffi::OsStr,
	io::Error as IoError,
};
use mockall::{Sequence, automock};



//		Traits																											

//§		Command																	
/// A mockable version of the [`std::process::Command`] trait.
/// 
/// # See also
/// 
/// * [`std::process::Command`]
/// 
#[automock]
pub trait Command {
	//		args																
	/// Adds multiple arguments to pass to the program.
	/// 
	/// # See also
	/// 
	/// * [`std::process::Command::args()`]
	/// 
	fn args(&self, args: Vec<String>);
	
	//		exec																
	/// Unix-specific extensions to the [`Command`] builder.
	/// 
	/// # See also
	/// 
	/// * [`std::os::unix::process::CommandExt::exec()`]
	/// 
	fn exec(&self) -> IoError;
	
	//		stdin																
	/// Configuration for the child process's standard input (`stdin`) handle.
	/// 
	/// # See also
	/// 
	/// * [`std::process::Command::stdin()`]
	/// 
	fn stdin(&self, cfg: MockStdio);
	
	//		stdout																
	/// Configuration for the child process's standard output (`stdout`) handle.
	/// 
	/// # See also
	/// 
	/// * [`std::process::Command::stdout()`]
	/// 
	fn stdout(&self, cfg: MockStdio);
	
	//		stderr																
	/// Configuration for the child process's standard error (`stderr`) handle.
	/// 
	/// # See also
	/// 
	/// * [`std::process::Command::stderr()`]
	/// 
	fn stderr(&self, cfg: MockStdio);
}



//		Structs																											

//		FakeCommand																
/// A mockable version of the [`std::process::Command`] struct.
/// 
/// This sets up the actual mock along with the expected sequence of calls, and
/// then interacts with the mocked functions, whilst returning itself for
/// chaining.
/// 
/// # See also
/// 
/// * [`std::process::Command`]
/// 
#[derive(Debug, Default)]
pub struct FakeCommand {
	/// The mock command. This is a stand-in for the real command instance, and
	/// will behave in the same way, but do nothing.
	command: MockCommand,
}

//󰭅		FakeCommand																
impl FakeCommand {
	//		new																	
	/// Creates a new instance of the [`FakeCommand`] struct.
	/// 
	/// # See also
	/// 
	/// * [`std::process::Command::new()`]
	/// 
	#[must_use]
	pub fn new<S: AsRef<OsStr>>(_program: S) -> Self {
		let mut sequence     = Sequence::new();
		let mut mock_command = MockCommand::new();
		_ = mock_command.expect_args()
			.withf(|list| list == &args().skip(1).collect::<Vec<_>>())
			.times(1)
			.in_sequence(&mut sequence)
			.returning(|_| ())
		;
		_ = mock_command.expect_stdin()
			.times(1)
			.in_sequence(&mut sequence)
			.returning(|_| ())
		;
		_ = mock_command.expect_stdout()
			.times(1)
			.in_sequence(&mut sequence)
			.returning(|_| ())
		;
		_ = mock_command.expect_stderr()
			.times(1)
			.in_sequence(&mut sequence)
			.returning(|_| ())
		;
		_ = mock_command.expect_exec()
			.times(1)
			.in_sequence(&mut sequence)
			.returning(|| IoError::from_raw_os_error(0))
		;
		Self {
			command: mock_command,
		}
	}
	
	//		args																
	/// Adds multiple arguments to pass to the program.
	/// 
	/// # See also
	/// 
	/// * [`std::process::Command::args()`]
	/// 
	pub fn args(&mut self, args: Vec<String>) -> &mut Self {
		self.command.args(args);
		self
	}
	
	//		exec																
	/// Unix-specific extensions to the [`Command`] builder.
	/// 
	/// # See also
	/// 
	/// * [`std::os::unix::process::CommandExt::exec()`]
	/// 
	pub fn exec(&mut self) -> IoError {
		self.command.exec()
	}
	
	//		stdin																
	/// Configuration for the child process's standard input (`stdin`) handle.
	/// 
	/// # See also
	/// 
	/// * [`std::process::Command::stdin()`]
	/// 
	pub fn stdin(&mut self, cfg: MockStdio) -> &mut Self {
		self.command.stdin(cfg);
		self
	}
	
	//		stdout																
	/// Configuration for the child process's standard output (`stdout`) handle.
	/// 
	/// # See also
	/// 
	/// * [`std::process::Command::stdout()`]
	/// 
	pub fn stdout(&mut self, cfg: MockStdio) -> &mut Self {
		self.command.stdout(cfg);
		self
	}
	
	//		stderr																
	/// Configuration for the child process's standard error (`stderr`) handle.
	/// 
	/// # See also
	/// 
	/// * [`std::process::Command::stderr()`]
	/// 
	pub fn stderr(&mut self, cfg: MockStdio) -> &mut Self {
		self.command.stderr(cfg);
		self
	}
}

//		MockStdio																
/// Mockable version of the standard input/output (stdio) configuration.
/// 
/// This is used to configure the standard input/output/error handles for the
/// child process.
/// 
/// # See also
/// 
/// * [`std::process::Stdio`]
/// 
#[derive(Debug)]
#[non_exhaustive]
pub struct MockStdio;

//󰭅		MockStdio																
#[expect(clippy::missing_const_for_fn, reason = "Needed for mock")]
impl MockStdio {
	//		inherit																
	/// Inherit the standard input/output (stdio) configuration.
	/// 
	/// # See also
	/// 
	/// * [`std::process::Stdio::inherit()`]
	/// 
	#[must_use]
	pub fn inherit() -> Self {
		Self
	}
}



//		Functions																										

//		mock_exit																
/// Mockable version of the [`std::process::exit()`] function.
/// 
/// # See also
/// 
/// * [`std::process::exit()`]
/// 
#[expect(clippy::missing_const_for_fn, reason = "Needed for mock")]
pub fn mock_exit(_code: i32) {}


