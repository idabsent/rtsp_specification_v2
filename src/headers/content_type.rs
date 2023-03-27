use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub struct ContentType {
    content_type: String,
}

impl ContentType {
    fn new(content_type: String) -> ContentType {
        ContentType {
            content_type,
        }
    }

    fn set_content_type(&mut self, content_type: String) {
        self.content_type = content_type;
    }
}

impl Header for ContentType {
    fn header() -> String {
        String::from("Content-Type")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Options,
          RequestMethod::Setup, RequestMethod::Play,
          RequestMethod::Pause, RequestMethod::Teardown,
          RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::Redirect, RequestMethod::PlayNotify]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::MessageBody
    }

    fn value(&self) -> String {
        self.content_type.clone()
    }
}