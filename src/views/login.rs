use axum::{
    http::{StatusCode, Uri},
    response::IntoResponse,
};
use hypertext::prelude::*;

use crate::views::page::Page;

#[derive(Debug, Default)]
pub struct LoginFormPage {
    prepopulated_email: Option<String>,
    show_invalid_credentials: bool,
    redirect_to: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct LoginFormPayload {
    pub email: String,
    pub password: String,
    pub redirect_to: Option<String>,
}

impl LoginFormPage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_prepopulated_email(mut self, email: impl ToString) -> Self {
        self.prepopulated_email = Some(email.to_string());
        self
    }

    pub fn show_invalid_credentials(mut self) -> Self {
        self.show_invalid_credentials = true;
        self
    }

    // pub fn set_redirect_to(mut self, redirect_to: impl ToString) -> Self {
    //     self.redirect_to = Some(redirect_to.to_string());
    //     self
    // }
    pub fn maybe_redirect_to(mut self, maybe_redirect: Option<String>) -> Self {
        self.redirect_to = maybe_redirect;
        self
    }
}

impl Renderable for LoginFormPage {
    fn render_to(&self, buffer: &mut hypertext::Buffer<hypertext::context::Node>) {
        maud! {
                Page title="Login" {
                   main {
                    form action="/login" method="POST" {
                        input type="email" name="email" placeholder= "email" required value=[&self.prepopulated_email];
                         input type="password" name="password" placeholder= "***" required;
                         @if let Some(redirect) = &self.redirect_to {
                            input type="hidden" name="redirect_to" value=(redirect);
                         }
                         @if self.show_invalid_credentials {
                            p class="text-red-500" { "Invalid email or password" }
                         }
                         button type="submit" { "Login" }
                    }
                   }
            }
        }
        .render_to(buffer);
    }
}

impl IntoResponse for LoginFormPage {
    fn into_response(self) -> axum::response::Response {
        let html = self.render();
        let status = if self.show_invalid_credentials {
            StatusCode::UNAUTHORIZED
        } else {
            StatusCode::OK
        };
        (status, html).into_response()
    }
}
