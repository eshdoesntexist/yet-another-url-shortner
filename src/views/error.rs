use axum::{http::StatusCode, response::IntoResponse};
use hypertext::prelude::*;

use crate::views::page::Page;

pub struct ErrorPage {
    status: axum::http::StatusCode,
    msg: String,
}

impl Default for ErrorPage {
    fn default() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            msg: "An unexpected error occurred".to_string(),
        }
    }
}

impl ErrorPage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn set_message(mut self, msg: impl ToString) -> Self {
        self.msg = msg.to_string();
        self
    }
}

impl Renderable for ErrorPage {
    fn render_to(&self, buffer: &mut hypertext::Buffer<hypertext::context::Node>) {
        maud! {
        Page title=(&format!("Error {}", self.msg)) {
            main
                class="grid min-h-full place-items-center bg-gray-900 px-6 py-24 sm:py-32 lg:px-8"
            {
                div class="text-center" {
                    p class="text-base font-semibold text-indigo-400" { (self.status.to_string()) }
                    h1
                        class="mt-4 text-5xl font-semibold tracking-tight text-balance text-white sm:text-7xl"
                    { "Error" }
                    p class="mt-6 text-lg font-medium text-pretty text-gray-400 sm:text-xl/8" {
                        (self.msg)
                    }
                    div class="mt-10 flex items-center justify-center gap-x-6" {
                        a
                            href="/"
                            class="rounded-md bg-indigo-500 px-3.5 py-2.5 text-sm font-semibold text-white shadow-xs hover:bg-indigo-400 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
                        { "Go back home" }
                    }
                }
            }
        }
    }
        .render_to(buffer);
    }
}
impl IntoResponse for ErrorPage {
    fn into_response(self) -> axum::response::Response {
        let html = self.render();
        let status = self.status;
        (status, html).into_response()
    }
}
