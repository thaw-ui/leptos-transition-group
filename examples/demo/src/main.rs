mod app;
mod transition;

use app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(App)
}
