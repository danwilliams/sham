//! This module mocks the [`reqwest`] crate in order to test HTTP requests.
//! 
//! This module mocks the critical parts of the [`reqwest`] crate using
//! [`mockall`], in order to test network functionality without actually sending
//! requests to a real HTTP server. This is important because unit tests should
//! not make actual network requests or rely upon having a real server running.
//! 
//! The approach taken is that the "real" code should import the [`Client`](reqwest::Client)
//! from [`reqwest`] when running in non-test mode, but import the mocked
//! `Client`, i.e. [`MockClient`], from this module when running in test mode.
//! This can be achieved by using conditional compilation. The test code can
//! then configure the mocks to expect certain requests and to return certain
//! responses, and then run the tests.
//! 
//! # Examples
//! 
//! ```rust
//! #[cfg(not(test))]
//! use reqwest::{Client, Error as ReqwestError, RequestBuilder, Response};
//! #[cfg(test)]
//! use sham::reqwest::{
//!     MockClient         as Client,
//!     MockError          as ReqwestError,
//!     MockRequestBuilder as RequestBuilder,
//!     MockResponse       as Response,
//! };
//! ```
//! 



//		Packages

use bytes::Bytes;
use core::{
	error::Error,
	fmt::{Debug, Display, Formatter, self},
	hash::BuildHasher,
	pin::Pin,
};
use futures_util::stream::{Stream, self};
#[allow(clippy::useless_attribute, reason = "Not useless! Here for the false positive")]
#[allow(clippy::allow_attributes,  reason = "False positive lint")]
#[allow(unused_imports,            reason = "False positive due to mocks")]
use mockall::{Sequence, concretize, mock};
use reqwest::{
	Body,
	IntoUrl,
	StatusCode,
	Url,
	header::{HeaderMap, HeaderName, CONTENT_LENGTH, CONTENT_TYPE},
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::from_slice as from_json_slice;
use std::{
	collections::HashMap,
	sync::Arc,
};



//		Mocks

//		Client																	
mock! {
	/// A mocked Reqwest client.
	/// 
	/// This is mocked by [`mockall`], and allows configuration of expected
	/// requests and responses for testing.
	/// 
	pub Client {
		//		delete															
		/// Creates a request builder for a `DELETE` request to the given URL.
		/// 
		/// # Parameters
		/// 
		/// * `url` - The URL to create a request builder for.
		/// 
		/// # See also
		/// 
		/// * [`reqwest::Client::delete()`]
		/// 
		#[concretize]
		pub fn delete<U: IntoUrl>(&self, url: U) -> MockRequestBuilder;
		
		//		get																
		/// Creates a request builder for a `GET` request to the given URL.
		/// 
		/// # Parameters
		/// 
		/// * `url` - The URL to create a request builder for.
		/// 
		/// # See also
		/// 
		/// * [`reqwest::Client::get()`]
		/// 
		#[concretize]
		pub fn get<U: IntoUrl>(&self, url: U) -> MockRequestBuilder;
		
		//		patch															
		/// Creates a request builder for a `PATCH` request to the given URL.
		/// 
		/// # Parameters
		/// 
		/// * `url` - The URL to create a request builder for.
		/// 
		/// # See also
		/// 
		/// * [`reqwest::Client::patch()`]
		/// 
		#[concretize]
		pub fn patch<U: IntoUrl>(&self, url: U) -> MockRequestBuilder;
		
		//		post															
		/// Creates a request builder for a `POST` request to the given URL.
		/// 
		/// # Parameters
		/// 
		/// * `url` - The URL to create a request builder for.
		/// 
		/// # See also
		/// 
		/// * [`reqwest::Client::post()`]
		/// 
		#[concretize]
		pub fn post<U: IntoUrl>(&self, url: U) -> MockRequestBuilder;
		
		//		put																
		/// Creates a request builder for a `PUT` request to the given URL.
		/// 
		/// # Parameters
		/// 
		/// * `url` - The URL to create a request builder for.
		/// 
		/// # See also
		/// 
		/// * [`reqwest::Client::put()`]
		/// 
		#[concretize]
		pub fn put<U: IntoUrl>(&self, url: U) -> MockRequestBuilder;
	}
	
	//󰭅		Clone																
	impl Clone for Client {
		//		clone															
		fn clone(&self) -> Self {
			Self {}
		}
	}
	
	//󰭅		Debug																
	impl Debug for Client {
		//		fmt																
		#[concretize]
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			write!(f, "Mocked Reqwest client")
		}
	}
}

//		RequestBuilder															
mock! {
	/// A mocked Reqwest request builder.
	/// 
	/// This is mocked by [`mockall`], and allows configuration of expected
	/// requests and responses for testing.
	/// 
	pub RequestBuilder {
		//		send															
		/// Sends the request and returns the response.
		/// 
		/// # See also
		/// 
		/// * [`reqwest::RequestBuilder::send()`]
		/// 
		pub async fn send(&self) -> Result<MockResponse, MockError>;
	}
}

//󰭅		RequestBuilder															
impl MockRequestBuilder {
	//		body																
	/// Set the request body.
	/// 
	/// Note, this is a supporting function in order to provide compatible
	/// functionality, and the mocked version actually does nothing.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::RequestBuilder::body()`]
	/// 
	#[expect(unused_mut,                     reason = "Needed for compatibility with the real Reqwest")]
	#[expect(clippy::needless_pass_by_value, reason = "Needed for compatibility with the real Reqwest")]
	#[must_use]
	pub fn body<T: Into<Body>>(mut self, _body: T) -> Self {
		self
	}
	
	//		form																
	/// Specify to send a form body.
	///
	/// Note, this is a supporting function in order to provide compatible
	/// functionality, and the mocked version actually does nothing.
	///
	/// # See also
	/// 
	/// * [`reqwest::RequestBuilder::form()`]
	/// 
	#[expect(unused_mut, reason = "Needed for compatibility with the real Reqwest")]
	#[must_use]
	pub const fn form<T: Serialize + ?Sized>(mut self, _form: &T) -> Self {
		self
	}
	
	//		headers																
	/// Adds headers to the request.
	///
	/// Note, this is a supporting function in order to provide compatible
	/// functionality, and the mocked version actually does nothing.
	///
	/// # See also
	/// 
	/// * [`reqwest::RequestBuilder::headers()`]
	/// 
	#[expect(unused_mut,                     reason = "Needed for compatibility with the real Reqwest")]
	#[expect(clippy::needless_pass_by_value, reason = "Needed for compatibility with the real Reqwest")]
	#[must_use]
	pub fn headers(mut self, _headers: HeaderMap) -> Self {
		self
	}
	
	//		json																
	/// Specify to send a JSON body.
	///
	/// Note, this is a supporting function in order to provide compatible
	/// functionality, and the mocked version actually does nothing.
	///
	/// # See also
	/// 
	/// * [`reqwest::RequestBuilder::json()`]
	/// 
	#[expect(unused_mut, reason = "Needed for compatibility with the real Reqwest")]
	#[must_use]
	pub const fn json<T: Serialize + ?Sized>(mut self, _json: &T) -> Self {
		self
	}
}



//		Structs

//		MockError																
/// A mocked error type for Reqwest.
/// 
/// Notably, the real [`reqwest::Error`] type cannot be created externally, and
/// has logic that depends on private types. Instead of trying to reproduce that
/// logic, this mock simply provides the means to specify what should be
/// returned when using the functions provided by the error type.
/// 
/// This is not mocked by [`mockall`], and is a simple supporting type.
/// 
#[expect(clippy::struct_excessive_bools, reason = "Acceptable here")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct MockError {
	//		Public properties													
	/// Whether the error is related to the request or response body.
	pub is_body:     bool,
	
	/// Whether the error is from a type builder.
	pub is_builder:  bool,
	
	/// Whether the error is related to making a connection.
	pub is_connect:  bool,
	
	/// Whether the error is related to decoding the response body.
	pub is_decode:   bool,
	
	/// Whether the error is from a redirect policy.
	pub is_redirect: bool,
	
	/// Whether the error is related to the request.
	pub is_request:  bool,
	
	/// Whether the error is from [`Response::error_for_status()`](reqwest::Response::error_for_status).
	pub is_status:   bool,
	
	/// Whether the error is related to a timeout.
	pub is_timeout:  bool,
	
	/// The status code, if the error was generated from a response.
	pub status:      Option<StatusCode>,
	
	/// A possible URL related to this error.
	pub url:         Option<Url>,
}

//󰭅		MockError																
impl MockError {
	//		is_body																
	/// Whether the error is related to the request or response body.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Error::is_body()`]
	/// 
	#[must_use]
	pub const fn is_body(&self) -> bool {
		self.is_body
	}
	
	//		is_builder															
	/// Whether the error is from a type builder.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Error::is_builder()`]
	/// 
	#[must_use]
	pub const fn is_builder(&self) -> bool {
		self.is_builder
	}
	
	//		is_connect															
	/// Whether the error is related to making a connection.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Error::status()`]
	/// 
	#[must_use]
	pub const fn is_connect(&self) -> bool {
		self.is_connect
	}
	
	//		is_decode															
	/// Whether the error is related to decoding the response body.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Error::is_decode()`]
	/// 
	#[must_use]
	pub const fn is_decode(&self) -> bool {
		self.is_decode
	}
	
	//		is_redirect															
	/// Whether the error is from a redirect policy.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Error::is_redirect()`]
	/// 
	#[must_use]
	pub const fn is_redirect(&self) -> bool {
		self.is_redirect
	}
	
	//		is_request															
	/// Whether the error is related to the request.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Error::is_request()`]
	/// 
	#[must_use]
	pub const fn is_request(&self) -> bool {
		self.is_request
	}
	
	//		is_status															
	/// Whether the error is from [`Response::error_for_status()`](reqwest::Response::error_for_status).
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Error::is_status()`]
	/// 
	#[must_use]
	pub const fn is_status(&self) -> bool {
		self.is_status
	}
	
	//		is_timeout															
	/// Whether the error is related to a timeout.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Error::is_timeout()`]
	/// 
	#[must_use]
	pub const fn is_timeout(&self) -> bool {
		self.is_timeout
	}
	
	//		status																
	/// The status code, if the error was generated from a response.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Error::status()`]
	/// 
	#[must_use]
	pub const fn status(&self) -> Option<StatusCode> {
		self.status
	}
	
	//		url																	
	/// Returns a possible URL related to this error.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Error::url()`]
	/// 
	#[must_use]
	pub const fn url(&self) -> Option<&Url> {
		self.url.as_ref()
	}
	
	//		url_mut																
	/// Returns a mutable reference to the URL related to this error.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Error::url_mut()`]
	/// 
	pub fn url_mut(&mut self) -> Option<&mut Url> {
		self.url.as_mut()
	}
	
	//		with_url															
	/// Add a URL related to this error (overwriting any existing).
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Error::with_url()`]
	/// 
	#[must_use]
	pub fn with_url(mut self, url: Url) -> Self {
		self.url = Some(url);
		self
	}
	
	//		without_url															
	/// Strip any related URL from this error.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Error::without_url()`]
	/// 
	#[must_use]
	pub fn without_url(mut self) -> Self {
		self.url = None;
		self
	}
}

//󰭅		Display																	
impl Display for MockError {
	//		fmt																	
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Mocked Reqwest error")
	}
}

//󰭅		Error																	
impl Error for MockError {}

//		MockResponse															
/// A mocked response type for Reqwest.
/// 
/// This is not mocked by [`mockall`], and is a simple supporting type.
/// 
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct MockResponse {
	//		Public properties													
	/// The URL of the response.
	pub url:     Url,
	
	/// The status code of the response.
	pub status:  StatusCode,
	
	/// The headers of the response.
	pub headers: HeaderMap,
	
	/// The body of the response.
	pub body:    Result<Arc<Bytes>, MockError>,
}

//󰭅		MockResponse															
#[expect(clippy::unused_async, reason = "Needed for compatibility with the real Reqwest")]
impl MockResponse {
	//		bytes																
	/// Returns the body of the response as a byte array.
	/// 
	/// # Errors
	/// 
	/// An error will be returned if there was a problem obtaining the body.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Response::bytes()`]
	/// 
	pub async fn bytes(&self) -> Result<Bytes, MockError> {
		self.body.clone().map(|bytes| (*bytes).clone())
	}
	
	//		bytes_stream														
	/// Returns the body of the response as a stream of byte arrays.
	/// 
	/// # Errors
	/// 
	/// An error will be returned if there was a problem obtaining the body.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Response::bytes_stream()`]
	/// 
	#[must_use]
	pub fn bytes_stream(&self) -> Pin<Box<dyn Stream<Item = Result<Bytes, MockError>> + Send>> {
		let body = self.body.clone();
		Box::pin(stream::once(async move { body.map(|bytes| (*bytes).clone()) }))
	}
	
	//		error_for_status													
	/// Turn a response into an error if the server returned an error.
	///
	/// # Errors
	/// 
	/// An error will be returned if the server returned an error.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Response::error_for_status()`]
	/// 
	pub fn error_for_status(self) -> Result<Self, MockError> {
		let status = self.status();
		if status.is_client_error() || status.is_server_error() {
			Err(MockError {
				is_status: true,
				status:    Some(status),
				url:       Some(self.url),
				..Default::default()
			})
		} else {
			Ok(self)
		}
	}
	
	//		error_for_status_ref												
	/// Turn a reference to a response into an error if the server returned an
	/// error.
	/// 
	/// # Errors
	/// 
	/// An error will be returned if the server returned an error.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Response::error_for_status_ref()`]
	/// 
	pub fn error_for_status_ref(&self) -> Result<&Self, MockError> {
		let status = self.status();
		if status.is_client_error() || status.is_server_error() {
			Err(MockError {
				is_status: true,
				status:    Some(status),
				url:       Some(self.url.clone()),
				..Default::default()
			})
		} else {
			Ok(self)
		}
	}
	
	//		headers																
	/// Returns the headers of the response.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Response::headers()`]
	/// 
	#[must_use]
	pub const fn headers(&self) -> &HeaderMap {
		&self.headers
	}
	
	//		json																
	/// Returns the body of the response deserialized from JSON to type `T`.
	/// 
	/// # Errors
	/// 
	/// An error will be returned if there was a problem obtaining the body, or
	/// if body is not in JSON format, or if it cannot be properly deserialized
	/// to target type `T`.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Response::json()`]
	/// 
	pub async fn json<T: DeserializeOwned>(&self) -> Result<T, MockError> {
		self.bytes().await.map(|bytes| from_json_slice(&bytes).unwrap())
	}
	
	//		status																
	/// Returns the status code of the response.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Response::status()`]
	/// 
	#[must_use]
	pub const fn status(&self) -> StatusCode {
		self.status
	}
	
	//		text																
	/// Returns the body of the response as a string.
	/// 
	/// # Errors
	/// 
	/// An error will be returned if there was a problem obtaining the body.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Response::text()`]
	/// 
	pub async fn text(&self) -> Result<String, MockError> {
		self.bytes().await.map(|bytes| String::from_utf8(bytes.to_vec()).unwrap())
	}
	
	//		url																	
	/// Returns the final URL of the response.
	/// 
	/// # See also
	/// 
	/// * [`reqwest::Response::url()`]
	/// 
	#[must_use]
	pub const fn url(&self) -> &Url {
		&self.url
	}
}



//		Functions

//		create_mock_client														
/// Creates a mock Reqwest client.
/// 
/// # Parameters
/// 
/// * `responses` - The responses to return for specific URLs. This is a list of
///                 tuples, where the first element is the expected URL, and the
///                 second element is the response to return.
/// 
#[must_use]
pub fn create_mock_client<U: IntoUrl>(responses: Vec<(U, Result<MockResponse, MockError>)>) -> MockClient {
	let mut mock_client = MockClient::new();
	let mut sequence    = Sequence::new();
	for (mock_url, mock_response) in responses {
		let expected_url: Url = mock_url.into_url().unwrap();
		_ = mock_client.expect_get()
			.withf(move |url| url.as_str() == expected_url.as_str())
			.times(1)
			.in_sequence(&mut sequence)
			.returning(move |_| {
				let mut mock_request    = MockRequestBuilder::new();
				let mock_response_clone = mock_response.clone();
				_ = mock_request.expect_send()
					.times(1)
					.returning(move || mock_response_clone.clone())
				;
				mock_request
			})
		;
	}
	mock_client
}

//		create_mock_response													
/// Creates a mock Reqwest response.
/// 
/// Notably, this function expects that the body is always a valid UTF-8 string.
/// 
/// # Parameters
/// 
/// * `url`           - The URL of the response.
/// * `status`        - The status code of the response.
/// * `content_type`  - The content type of the response.
/// * `content_len`   - The content length of the response.
/// * `extra_headers` - Any additional headers to include in the response.
/// * `body`          - The body of the response.
/// 
pub fn create_mock_response<U, S1, S2, S3, H: BuildHasher>(
	url:           U,
	status:        StatusCode,
	content_type:  Option<S1>,
	content_len:   Option<usize>,
	extra_headers: HashMap<S2, S3, H>,
	body:          Result<&[u8], MockError>,
) -> MockResponse
where
	U:  IntoUrl,
	S1: Into<String>,
	S2: Into<String>,
	S3: Into<String>,
{
	MockResponse {
		url:     url.into_url().unwrap(),
		status,
		headers: {
			let mut headers = HeaderMap::new();
			if let Some(ct) = content_type {
				drop(headers.insert(CONTENT_TYPE, ct.into().parse().unwrap()));
			}
			if let Some(cl) = content_len {
				drop(headers.insert(CONTENT_LENGTH, format!("{cl}").parse().unwrap()));
			}
			headers.extend(extra_headers.into_iter().map(|(k, v)|
				(k.into().parse::<HeaderName>().unwrap(), v.into().parse().unwrap())
			));
			headers
		},
		body:    body.map(|bytes| Arc::new(Bytes::copy_from_slice(bytes))),
	}
}


