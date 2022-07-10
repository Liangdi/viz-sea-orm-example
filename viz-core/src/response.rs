use crate::{header, types::PayloadError, Body, Response, Result, StatusCode};

pub trait ResponseExt {
    /// Response body with `Content-Type`
    fn with<T>(t: T, c: &'static str) -> Response<Body>
    where
        T: Into<Body>,
    {
        let mut res = Response::new(t.into());
        res.headers_mut()
            .insert(header::CONTENT_TYPE, header::HeaderValue::from_static(c));
        res
    }

    /// Response TEXT
    fn text<T>(t: T) -> Response<Body>
    where
        T: Into<Body>,
    {
        Self::with(t, mime::TEXT_PLAIN_UTF_8.as_ref())
    }

    /// Response HTML
    fn html<T>(t: T) -> Response<Body>
    where
        T: Into<Body>,
    {
        Self::with(t, mime::TEXT_HTML_UTF_8.as_ref())
    }

    #[cfg(feature = "json")]
    /// Response JSON
    fn json<T>(t: T) -> Result<Response<Body>, PayloadError>
    where
        T: serde::Serialize,
    {
        serde_json::to_vec(&t)
            .map(|v| Self::with(v, mime::APPLICATION_JSON.as_ref()))
            .map_err(PayloadError::Json)
    }

    // TODO: Download transfers the file from path as an attachment.
    // fn download() -> Response<Body>

    /// The response was successful (status in the range [`200-299`][mdn]) or not.
    ///
    /// [mdn]: <https://developer.mozilla.org/en-US/docs/Web/API/Response/ok>
    fn ok(&self) -> bool;

    fn location(location: &'static str) -> Self;

    /// The response redirects to the specified URL.
    ///
    /// [mdn]: <https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect>
    fn redirect<T>(url: T) -> Response<Body>
    where
        T: AsRef<str>;

    /// The response redirects to the specified URL and the status code.
    ///
    /// [mdn]: <https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect>
    fn redirect_with_status<T>(uri: T, status: StatusCode) -> Response<Body>
    where
        T: AsRef<str>;

    /// <https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/303>
    fn see_other<T>(url: T) -> Response<Body>
    where
        T: AsRef<str>,
    {
        Self::redirect_with_status(url, StatusCode::SEE_OTHER)
    }

    /// <https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/307>
    fn temporary<T>(url: T) -> Response<Body>
    where
        T: AsRef<str>,
    {
        Self::redirect_with_status(url, StatusCode::TEMPORARY_REDIRECT)
    }

    /// <https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/308>
    fn permanent<T>(url: T) -> Response<Body>
    where
        T: AsRef<str>,
    {
        Self::redirect_with_status(url, StatusCode::PERMANENT_REDIRECT)
    }
}

impl ResponseExt for Response<Body> {
    fn ok(&self) -> bool {
        self.status().is_success()
    }

    fn location(location: &'static str) -> Self {
        let mut res = Self::default();
        res.headers_mut().insert(
            header::CONTENT_LOCATION,
            header::HeaderValue::from_static(location),
        );
        res
    }

    fn redirect<T>(url: T) -> Response<Body>
    where
        T: AsRef<str>,
    {
        match header::HeaderValue::try_from(url.as_ref()) {
            Ok(val) => {
                let mut res = Self::default();
                res.headers_mut().insert(header::LOCATION, val);
                res
            }
            Err(err) => panic!("{}", err),
        }
    }

    fn redirect_with_status<T>(url: T, status: StatusCode) -> Response<Body>
    where
        T: AsRef<str>,
    {
        assert!(status.is_redirection(), "not a redirection status code");

        let mut res = Self::redirect(url);
        *res.status_mut() = status;
        res
    }
}