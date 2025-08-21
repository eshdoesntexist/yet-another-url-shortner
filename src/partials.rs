use maud::{DOCTYPE, Markup, html};

use crate::url_store::ShortUrlRow;

pub fn page<'a>(title: &'a str, children: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { (title) }
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                // TODO for now unpkg is used, but this should be replaced with a local copy in the future
                script src="https://unpkg.com/htmx.org" {}
                // TODO probably figure out a way clean way to use the tailwnindcss cli to also decrease the bundle size
                script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {}
            }
            body class="bg-gray-900 text-gray-100" { (children) }
        }
    }
}

pub fn url_table(values: Vec<ShortUrlRow>) -> Markup {
    html! {
        main class="container mx-auto mt-10" {
            section
                class="w-full mb-10 mx-auto shadow-md sm:rounded-lg p-2 bg-white border dark:bg-gray-800 dark:border-gray-700 border-gray-200"
            {
                form class="flex flex-row gap-2" method="post" action="/add" {
                    label
                        for="add-url"
                        class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white"
                    { "add" }
                    input
                        id="add-url"
                        class="block w-full p-4 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        placeholder="Add a new URL"
                        name="url"
                        required;
                    button
                        type="Add url"
                        class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-8 py-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                    { "Add" }
                }
            }
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
                    tbody {
                        @for row in &values { (url_table_row(row)) }
                    }
                }
            }
        }
    }
}

pub fn url_table_row(row: &ShortUrlRow) -> Markup {
    html! {
        tr
            class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 border-gray-200 hover:bg-gray-50 dark:hover:bg-gray-600"
        {
            th
                scope="row"
                class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white"
            { (row.shorturl) }
            td class="px-6 py-4" {
                a href=(row.longurl) target="_blank" { (row.longurl) }
            }
            td class="px-6 py-4" { (row.created_at) }
        }
    }
}
