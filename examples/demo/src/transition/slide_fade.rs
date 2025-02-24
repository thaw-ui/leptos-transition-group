use leptos::prelude::*;
use leptos_meta::Style;
use leptos_transition_group::CSSTransition;

#[component]
pub fn SlideFade() -> impl IntoView {
    let show = RwSignal::new(false);

    view! {
        <thaw::Card>
            <thaw::CardHeader>"slide-fade"</thaw::CardHeader>
            <Style>
                ".slide-fade-enter-active {
                    transition: all 0.3s ease-out;
                }
                
                .slide-fade-leave-active {
                    transition: all 0.8s cubic-bezier(1, 0.5, 0.8, 1);
                }
                
                .slide-fade-enter-from,
                .slide-fade-leave-to {
                    transform: translateX(20px);
                    opacity: 0;
                }"
            </Style>
            <thaw::Button on_click=move |_| {
                show.update(|v| *v = !*v)
            }>"Toggle Slide + Fade"</thaw::Button>
            <CSSTransition show=show name="slide-fade">
                <p>"hello"</p>
            </CSSTransition>
        </thaw::Card>
    }
}
