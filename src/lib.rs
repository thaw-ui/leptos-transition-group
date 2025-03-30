mod get_transition_info;
mod use_next_frame;
mod utils;

use get_transition_info::{get_transition_info, AnimationTypes, CSSTransitionInfo};
use leptos::{
    ev,
    prelude::*,
    tachys::html::{node_ref::node_ref, style::style},
};
use send_wrapper::SendWrapper;
use std::{ops::Deref, time::Duration};
use use_next_frame::NextFrame;
use utils::{add_event_listener, ArcOneCallback, EventListenerHandle};
use web_sys::HtmlElement;

/// # CSSTransition
///
/// It can be used to apply enter and leave animations on elements or
/// components passed to it via its default slot.
///
/// ## TIP
///
/// `CSSTransition` only supports a single element or component as its
/// slot content. If the content is a component, the component must
/// also have only one single root element.
///
/// ## Examples
///
/// This is an example of the most basic usage:
///
/// ``` rust
/// use leptos::prelude::*;
/// let show = RwSignal::new(false);
/// view!{
///     <button on:click=move |_| show.update(|v| *v = !*v)>"Toggle"</button>
///     <CSSTransition
///         show=show
///         name="fade"
///     >
///         <p>"hello"</p>
///     </CSSTransition>
/// }
/// ```
///
/// Add the following CSS:
///
/// ``` css
/// .fade-enter-active,
/// .fade-leave-active {
///     transition: opacity 0.5s ease;
/// }

/// .fade-enter-from,
/// .fade-leave-to {
///     opacity: 0;
/// }
/// ```
#[component]
pub fn CSSTransition<T>(
    #[prop(into)] show: Signal<bool>,
    /// Used to automatically generate transition CSS class names.
    /// e.g. `name: 'fade'` will auto expand to `.fade-enter`,
    /// `.fade-enter-active`, etc.
    #[prop(into)]
    name: Signal<String>,
    /// Whether to apply transition on initial render.
    #[prop(optional)]
    appear: bool,
    #[prop(optional, into)] on_before_enter: Option<ArcOneCallback<HtmlElement>>,
    #[prop(optional, into)] on_enter: Option<ArcOneCallback<HtmlElement>>,
    #[prop(optional, into)] on_after_enter: Option<ArcOneCallback<HtmlElement>>,
    #[prop(optional, into)] on_before_leave: Option<ArcOneCallback<HtmlElement>>,
    #[prop(optional, into)] on_leave: Option<ArcOneCallback<HtmlElement>>,
    #[prop(optional, into)] on_after_leave: Option<ArcOneCallback<HtmlElement>>,
    children: TypedChildren<T>,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    let target_ref = NodeRef::<utils::HtmlElement>::new();
    let next_frame = NextFrame::new();
    let end_handle = StoredValue::new(None::<EventListenerHandle>);
    let end_count = StoredValue::new(None::<usize>);
    let finish = StoredValue::new(None::<Box<dyn FnOnce() + Send + Sync>>);

    Effect::new(move |_| {
        let Some(el) = target_ref.get() else {
            return;
        };

        let class_list = el.class_list();
        let style = el.style();

        let on_end = {
            let el = send_wrapper::SendWrapper::new(el.clone());
            move |remove: Box<dyn FnOnce() + Send + Sync>| {
                let Some(CSSTransitionInfo {
                    types,
                    prop_count,
                    timeout,
                }) = get_transition_info(&el)
                else {
                    remove();
                    return;
                };

                finish.set_value(Some(Box::new(move || {
                    end_count.set_value(None);
                    remove();
                    end_handle.update_value(|h| {
                        h.take().map(|h| {
                            h.remove();
                        });
                    });
                })));

                set_timeout(
                    move || {
                        if let Some(Some(f)) = finish.try_update_value(|f| f.take()) {
                            f();
                        }
                    },
                    Duration::from_millis(timeout + 1),
                );

                end_count.set_value(Some(0));
                let event_listener = move || {
                    end_count.update_value(|v| {
                        let Some(v) = v else {
                            return;
                        };
                        *v += 1;
                    });
                    if end_count.with_value(|v| {
                        let Some(v) = v else {
                            return false;
                        };
                        *v >= prop_count
                    }) {
                        if let Some(Some(f)) = finish.try_update_value(|f| f.take()) {
                            f();
                        }
                    }
                };
                let handle = match types {
                    AnimationTypes::Transition => {
                        add_event_listener(el.deref().clone(), ev::transitionend, move |_| {
                            event_listener()
                        })
                    }
                    AnimationTypes::Animation => {
                        add_event_listener(el.deref().clone(), ev::animationend, move |_| {
                            event_listener()
                        })
                    }
                };
                end_handle.set_value(Some(handle));
            }
        };

        let on_finish = move || {
            if let Some(Some(f)) = finish.try_update_value(|f| f.take()) {
                f();
            }
        };

        let name = name.clone();
        let on_before_enter = on_before_enter.clone();
        let on_enter = on_enter.clone();
        let on_after_enter = on_after_enter.clone();
        let on_before_leave = on_before_leave.clone();
        let on_leave = on_leave.clone();
        let on_after_leave = on_after_leave.clone();
        let effect = RenderEffect::new(move |prev: Option<bool>| {
            let show = show.get();
            let prev = if let Some(prev) = prev {
                prev
            } else if show && appear {
                false
            } else {
                if show {
                    let _ = style.set_property("display", "");
                } else {
                    let _ = style.set_property("display", "none");
                }
                return show;
            };

            let name = name.get_untracked();

            if show && !prev {
                on_finish();
                {
                    // on_enter
                    if let Some(on_before_enter) = on_before_enter.as_ref() {
                        on_before_enter(el.clone());
                    }

                    let enter_from = format!("{name}-enter-from");
                    let enter_active = format!("{name}-enter-active");
                    let enter_to = format!("{name}-enter-to");

                    // enter-from enter-active
                    let _ = class_list.add_2(&enter_from, &enter_active);
                    let _ = style.set_property("display", "");

                    let class_list = class_list.clone();
                    let on_end = on_end.clone();
                    let on_enter = on_enter.clone();
                    let on_after_enter = on_after_enter.clone();
                    let el = el.clone();
                    next_frame.run(move || {
                        // enter-active enter-to
                        let _ = class_list.remove_1(&enter_from);
                        let _ = class_list.add_1(&enter_to);

                        let remove = {
                            let class_list = SendWrapper::new(class_list);
                            let el = SendWrapper::new(el.clone());
                            Box::new(move || {
                                let _ = class_list.remove_2(&enter_active, &enter_to);
                                if let Some(on_after_enter) = on_after_enter.as_ref() {
                                    on_after_enter(el.deref().clone());
                                }
                            })
                        };
                        on_end(remove);

                        if let Some(on_enter) = on_enter.as_ref() {
                            on_enter(el);
                        }
                    });
                }
            } else if !show && prev {
                on_finish();
                {
                    // on_leave
                    if let Some(on_before_leave) = on_before_leave.as_ref() {
                        on_before_leave(el.clone());
                    }

                    let leave_from = format!("{name}-leave-from");
                    let leave_active = format!("{name}-leave-active");
                    let leave_to = format!("{name}-leave-to");

                    // leave-from leave-active
                    let _ = class_list.add_2(&leave_from, &leave_active);

                    let class_list = class_list.clone();
                    let style = style.clone();
                    let on_end = on_end.clone();
                    let on_leave = on_leave.clone();
                    let on_after_leave = on_after_leave.clone();
                    let el = el.clone();
                    next_frame.run(move || {
                        // leave-active leave-to
                        let _ = class_list.remove_1(&leave_from);
                        let _ = class_list.add_1(&leave_to);

                        let remove = {
                            let class_list = SendWrapper::new(class_list);
                            let style = SendWrapper::new(style);
                            let el = SendWrapper::new(el.clone());
                            Box::new(move || {
                                let _ = class_list.remove_2(&leave_active, &leave_to);
                                let _ = style.set_property("display", "none");
                                if let Some(on_after_leave) = on_after_leave.as_ref() {
                                    on_after_leave(el.deref().clone());
                                }
                            })
                        };
                        on_end(remove);

                        if let Some(on_leave) = on_leave {
                            on_leave(el);
                        }
                    });
                }
            }

            show
        });

        on_cleanup(move || {
            drop(effect);
            end_handle.update_value(|handle| {
                if let Some(handle) = handle.take() {
                    handle.remove();
                }
            });
        })
    });

    children.into_inner()()
        .into_inner()
        .add_any_attr(style((
            "display",
            if show.get_untracked() { "" } else { "none" },
        )))
        .add_any_attr(node_ref(target_ref))
}
