use chrono::NaiveDate;
use gloo_console::__macro::JsValue;
use web_sys::{Event, HtmlInputElement};
use yew::{Callback, UseStateHandle};

pub fn date_selector_onchange(use_state: &UseStateHandle<Option<NaiveDate>>) -> Callback<Event> {
    let use_state = use_state.clone();
    Callback::from(move |e: Event| {
        let result = HtmlInputElement::from(JsValue::from(e.target().unwrap().value_of())).value();
        use_state.set(Some(
            NaiveDate::parse_from_str(&result, "%Y-%m-%d").unwrap(),
        ));
    })
}

pub fn number_selector_onchange(use_state: &UseStateHandle<usize>) -> Callback<Event> {
    let use_state = use_state.clone();
    Callback::from(move |e: Event| {
        let result = HtmlInputElement::from(JsValue::from(e.target().unwrap().value_of())).value();
        use_state.set(result.parse::<usize>().unwrap());
    })
}
