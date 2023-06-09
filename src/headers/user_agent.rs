use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub fn user_agent_helper() -> String {
    String::from("User-Agent")
}

pub struct UserAgent {
    agent_name: String,
    version: f32,
}

impl UserAgent {
    fn new(agent_name: String, version: f32) -> UserAgent {
        if version.is_sign_negative() {
            panic!("Version number must be positive number");
        }

        UserAgent {
            agent_name,
            version,
        }
    }

    fn set_server_name(&mut self, agent_name: String) {
        self.agent_name = agent_name;
    }

    fn server_version(&mut self, version: f32) {
        self.version = version;
    }
}

impl Header for UserAgent {
    fn header(&self) -> String {
        user_agent_helper()
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Setup,
          RequestMethod::Play, RequestMethod::Options,
          RequestMethod::Pause, RequestMethod::Teardown,
          RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::Redirect, RequestMethod::PlayNotify]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        format!("{}/{}", self.agent_name, self.version)
    }
}