//! Event Message
//!
//! [mdn]: <https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#event_stream_format>

use bytes::Bytes;
use std::fmt::{self, Write};

#[derive(Default)]
pub struct Event {
    id: Option<String>,
    data: Option<String>,
    event: Option<String>,
    retry: Option<u64>,
    comment: Option<String>,
}

impl Event {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id.replace(id.into());
        self
    }

    pub fn data(mut self, data: impl Into<String>) -> Self {
        self.data.replace(data.into());
        self
    }

    pub fn event(mut self, event: impl Into<String>) -> Self {
        self.event.replace(event.into());
        self
    }

    pub fn retry(mut self, retry: u64) -> Self {
        self.retry.replace(retry);
        self
    }

    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment.replace(comment.into());
        self
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(comment) = &self.comment {
            ":".fmt(f)?;
            comment.fmt(f)?;
            f.write_char('\n')?;
        }
        if let Some(event) = &self.event {
            "event:".fmt(f)?;
            event.fmt(f)?;
            f.write_char('\n')?;
        }
        if let Some(data) = &self.data {
            for line in data.lines() {
                "data: ".fmt(f)?;
                line.fmt(f)?;
                f.write_char('\n')?;
            }
        }
        if let Some(id) = &self.id {
            "id:".fmt(f)?;
            id.fmt(f)?;
            f.write_char('\n')?;
        }
        if let Some(millis) = self.retry {
            "retry:".fmt(f)?;
            millis.fmt(f)?;
            f.write_char('\n')?;
        }
        f.write_char('\n')
    }
}

impl From<Event> for Bytes {
    fn from(e: Event) -> Self {
        Bytes::from(e.to_string())
    }
}
