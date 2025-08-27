use axum::response::IntoResponse;
use hypertext::{Raw, prelude::*};

use crate::views::page::Page;

use crate::url_store::ShortUrlRow as ShortUrlRowModel;

#[derive(Default, Debug)]
pub struct DashboardPageBuilder {
    rows: Vec<ShortUrlRowModel>,
}

impl DashboardPageBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_rows(mut self, rows: Vec<ShortUrlRowModel>) -> Self {
        self.rows = rows;
        self
    }
}

const UP_ARROW_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"> <path stroke-linecap="round" stroke-linejoin="round" d="M5 15l7-7 7 7"/></svg>"#;

impl Renderable for DashboardPageBuilder {
    fn render_to(&self, buffer: &mut hypertext::Buffer<hypertext::context::Node>) {
        maud! {
            Page title="Dashboard" {
                div
                    id="scrollTopBtn"
                    class="hidden fixed bottom-6 right-6 bg-blue-600 text-white p-3 rounded-full shadow-lg cursor-pointer hover:bg-blue-700 transition"
                { (Raw::dangerously_create(UP_ARROW_SVG)) }

                main class="container mx-auto mt-10" {
                    AddUrlForm;
                    section class="relative overflow-x-auto shadow-md sm:rounded-lg" {
                        table
                            class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400"
                        {
                            thead
                                class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400"
                            {
                                tr {
                                    th class="px-6 py-3" { "Url" }
                                    th class="px-6 py-3" { "Redirects To" }
                                    th class="px-6 py-3" { "Created At" }
                                }
                            }
                            tbody #urltablebody {
                                @for row in &self.rows {
                                    UrlTableRow data=(row) hx=(false);
                                }
                            }
                        }
                    }
                }
            }
        }.render_to(buffer);
    }
}

impl IntoResponse for DashboardPageBuilder {
    fn into_response(self) -> axum::response::Response {
        self.render().into_response()
    }
}

#[component]
pub fn add_url_form() -> impl Renderable {
    maud! {
        section
            class="w-full mb-10 mx-auto shadow-md sm:rounded-lg p-2 bg-white border dark:bg-gray-800 dark:border-gray-700 border-gray-200"
        {
            form
                class="flex flex-row gap-2"
                method="post"
                action="/add"
                hx-post="/add"
                hx-target="#urltablebody"
                hx-swap="afterbegin"
                hx-on::after-request="this.reset(); this.querySelectorAll('input, button').forEach(el => el.disabled = false);"
                hx-on::before-request="this.querySelectorAll('input, button').forEach(el => el.disabled = true);"
            {
                label
                    for="add-url"
                    class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white"
                { "add" }
                input
                    id="add-url"
                    class="disabled:opacity-50 disabled:cursor-not-allowed block w-full p-4 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    placeholder="Add a new URL"
                    name="url"
                    required;
                button
                    type="Add url"
                    class="disabled:opacity-50 disabled:cursor-not-allowed text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-8 py-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800 "
                { "Add" }
            }
        }
    }
}

pub struct UrlTableRow<'a> {
    data: &'a ShortUrlRowModel,
    hx: bool,
}

impl<'a> UrlTableRow<'a> {
    pub fn new(row: &'a ShortUrlRowModel) -> Self {
        Self {
            hx: false,
            data: row,
        }
    }
    pub fn is_hx(mut self, val: bool) -> Self {
        self.hx = val;
        self
    }
}

impl<'a> IntoResponse for UrlTableRow<'a> {
    fn into_response(self) -> axum::response::Response {
        self.render().into_response()
    }
}

impl<'a> Renderable for UrlTableRow<'a> {
    fn render_to(&self, buffer: &mut hypertext::Buffer<hypertext::context::Node>) {
        let row = &self.data;
        maud! {
            tr
                class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 border-gray-200 hover:bg-gray-50 dark:hover:bg-gray-600"
            {
                th
                    scope="row"
                    class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white data-time"
                { (row.shorturl) }
                td class="px-6 py-4" {
                    a href=(row.longurl) target="_blank" { (row.longurl) }
                }
                td class="px-6 py-4" { (row.created_at.to_string()) }
            }
        }.render_to(buffer);
    }
}
