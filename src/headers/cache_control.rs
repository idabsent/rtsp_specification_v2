use std::{
    fmt,
    string::ToString,
};

use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::{RequestMethod},
};

pub enum CacheMethod {
    NoCache,
    Public,
    Private,
    NoTransform,
    OnlyIfCached,
    MaxStale,
    MinFresh,
    MustRevalidate,
    ProxyRevalidate,
    MaxAge,
}

impl fmt::Display for CacheMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let method = match self {
            Self::NoCache => "no-cache",
            Self::Public => "public",
            Self::Private => "private",
            Self::NoTransform => "transform",
            Self::OnlyIfCached => "only-if-cached",
            Self::MaxStale => "max-stale",
            Self::MinFresh => "min-fresh",
            Self::MustRevalidate => "must-revalidate",
            Self::ProxyRevalidate => "proxy-revalidate",
            Self::MaxAge => "max-age",
        };

        write!(f, "{method}")
    }
}

fn cache_control_helper() -> String {
    String::from("Cache-Control")
}

pub struct CacheControl {
    cache_method: CacheMethod,
}

impl CacheControl {
    fn new(cache_method: CacheMethod) -> CacheControl {
        CacheControl {
            cache_method,
        }
    }

    fn set_cache_method(&mut self, cache_method: CacheMethod) {
        self.cache_method = cache_method;
    }
}

impl Header for CacheControl {
    fn header(&self) -> String {
        cache_control_helper()
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::Describe, RequestMethod::Setup]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        self.cache_method.to_string()
    }
}