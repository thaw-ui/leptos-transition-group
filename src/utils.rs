use leptos::{ev, html::ElementType};
use send_wrapper::SendWrapper;
use std::{ops::Deref, sync::Arc};
use web_sys::{
    wasm_bindgen::{prelude::Closure, JsCast},
    EventTarget,
};

#[derive(Clone)]
pub struct ArcOneCallback<A, Return = ()>(Arc<dyn Fn(A) -> Return + Send + Sync + 'static>);

impl<A, Return> ArcOneCallback<A, Return> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A) -> Return + Send + Sync + 'static,
    {
        Self(Arc::new(f))
    }
}

impl<A, Return> Deref for ArcOneCallback<A, Return> {
    type Target = Arc<dyn Fn(A) -> Return + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F, A, Return> From<F> for ArcOneCallback<A, Return>
where
    F: Fn(A) -> Return + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

pub fn add_event_listener<E>(
    target: impl Into<EventTarget>,
    event: E,
    cb: impl Fn(E::EventType) + 'static,
) -> EventListenerHandle
where
    E: ev::EventDescriptor + 'static,
    E::EventType: JsCast,
{
    add_event_listener_untyped(target, &event.name(), move |e| {
        cb(e.unchecked_into::<E::EventType>())
    })
}

pub struct EventListenerHandle(Box<dyn FnOnce() + Send + Sync>);

impl std::fmt::Debug for EventListenerHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EventListenerHandle").finish()
    }
}

impl EventListenerHandle {
    pub fn remove(self) {
        (self.0)();
    }
}

fn add_event_listener_untyped(
    target: impl Into<EventTarget>,
    event_name: &str,
    cb: impl Fn(web_sys::Event) + 'static,
) -> EventListenerHandle {
    fn wel(
        target: EventTarget,
        cb: Box<dyn FnMut(web_sys::Event)>,
        event_name: &str,
    ) -> EventListenerHandle {
        let cb = Closure::wrap(cb);
        _ = target.add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref());

        EventListenerHandle({
            let event_name = event_name.to_string();
            let cb = SendWrapper::new(cb);
            let target = SendWrapper::new(target);
            Box::new(move || {
                let _ = target
                    .remove_event_listener_with_callback(&event_name, cb.as_ref().unchecked_ref());
            })
        })
    }

    wel(target.into(), Box::new(cb), event_name)
}

#[derive(Debug, Clone)]
pub struct HtmlElement {
    el: SendWrapper<web_sys::HtmlElement>,
}

impl ElementType for HtmlElement {
    type Output = web_sys::HtmlElement;

    const TAG: &'static str = "";

    const SELF_CLOSING: bool = false;

    const ESCAPE_CHILDREN: bool = false;

    const NAMESPACE: Option<&'static str> = None;

    fn tag(&self) -> &str {
        ""
    }
}

impl Deref for HtmlElement {
    type Target = web_sys::HtmlElement;

    fn deref(&self) -> &Self::Target {
        &self.el
    }
}
