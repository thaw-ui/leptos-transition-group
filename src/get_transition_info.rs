use leptos::prelude::window;

#[derive(PartialEq)]
pub enum AnimationTypes {
    Transition,
    Animation,
}

pub struct CSSTransitionInfo {
    pub types: AnimationTypes,
    pub prop_count: usize,
    pub timeout: u64,
}

pub fn get_transition_info(el: &web_sys::Element) -> Option<CSSTransitionInfo> {
    let styles = window().get_computed_style(el).ok().flatten()?;

    let get_style_properties = |property: &str| {
        styles
            .get_property_value(property)
            .unwrap_or_default()
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
    };

    let transition_delays = get_style_properties("transition-delay");
    let transition_durations = get_style_properties("transition-duration");
    let transition_timeout = get_timeout(transition_delays, &transition_durations);
    let animation_delays = get_style_properties("animation-delay");
    let animation_durations = get_style_properties("animation-duration");
    let animation_timeout = get_timeout(animation_delays, &animation_durations);

    let timeout = u64::max(transition_timeout, animation_timeout);
    let (types, prop_count) = if timeout > 0 {
        if transition_timeout > animation_timeout {
            (AnimationTypes::Transition, transition_durations.len())
        } else {
            (AnimationTypes::Animation, animation_durations.len())
        }
    } else {
        return None;
    };

    Some(CSSTransitionInfo {
        types,
        prop_count,
        timeout,
    })
}

fn get_timeout(mut delays: Vec<String>, durations: &Vec<String>) -> u64 {
    while delays.len() < durations.len() {
        delays.append(&mut delays.clone())
    }

    fn to_ms(s: &String) -> u64 {
        if s == "auto" || s.is_empty() {
            return 0;
        }

        let s = if s.ends_with("ms") {
            s.split_at(s.len() - 2).0
        } else {
            s.split_at(s.len() - 1).0
        };

        (s.parse::<f32>().unwrap_or_default() * 1000.0).floor() as u64
    }

    durations
        .iter()
        .enumerate()
        .map(|(i, d)| to_ms(d) + to_ms(&delays[i]))
        .max()
        .unwrap_or_default()
}
