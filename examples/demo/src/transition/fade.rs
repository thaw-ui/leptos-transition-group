use leptos::prelude::*;
use leptos_meta::Style;
use leptos_transition_group::CSSTransition;

#[component]
pub fn Fade() -> impl IntoView {
    let show = RwSignal::new(false);

    view! {
        <thaw::Card>
            <thaw::CardHeader>"Basic"</thaw::CardHeader>
            <Style>
                ".fade-enter-active,
                .fade-leave-active {
                    transition: opacity 0.5s ease;
                }
                
                .fade-enter-from,
                .fade-leave-to {
                    opacity: 0;
                }"
            </Style>
            <thaw::Button on_click=move |_| show.update(|v| *v = !*v)>"Toggle"</thaw::Button>
            <CSSTransition show=show name="fade">
                <p>"hello"</p>
            </CSSTransition>
        </thaw::Card>
    }
}
