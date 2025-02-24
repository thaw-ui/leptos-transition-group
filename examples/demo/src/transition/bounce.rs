use leptos::prelude::*;
use leptos_meta::Style;
use leptos_transition_group::CSSTransition;

#[component]
pub fn Bounce() -> impl IntoView {
    let show = RwSignal::new(false);

    view! {
        <thaw::Card>
            <thaw::CardHeader>"bounce"</thaw::CardHeader>
            <Style>
                ".bounce-enter-active {
                    animation: bounce-in 0.5s;
                }
                .bounce-leave-active {
                    animation: bounce-in 0.5s reverse;
                }
                @keyframes bounce-in {
                    0% {
                        transform: scale(0);
                    }
                    50% {
                        transform: scale(1.25);
                    }
                    100% {
                        transform: scale(1);
                    }
                }"
            </Style>
            <thaw::Button on_click=move |_| show.update(|v| *v = !*v)>"Toggle"</thaw::Button>
            <CSSTransition show=show name="bounce">
                <p style="text-align: center;">"Hello here is some bouncy text!"</p>
            </CSSTransition>
        </thaw::Card>
    }
}
