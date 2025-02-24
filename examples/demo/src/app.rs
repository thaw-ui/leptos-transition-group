use super::transition::{bounce::Bounce, fade::Fade, slide_fade::SlideFade};
use leptos::prelude::*;
use leptos_meta::Style;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <thaw::ConfigProvider>
            <Style>
                "main {
                    margin: 20px auto;
                    width: 860px;
                }"
            </Style>
            <main>
                <h1>"CSSTransition"</h1>
                <thaw::Flex vertical=true>
                    <Fade />
                    <SlideFade />
                    <Bounce />
                </thaw::Flex>
            </main>
        </thaw::ConfigProvider>
    }
}
