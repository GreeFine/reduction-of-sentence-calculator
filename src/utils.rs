use chrono::NaiveDate;
use gloo_console::__macro::JsValue;
use web_sys::{Event, HtmlInputElement};
use yew::{Callback, UseStateHandle};

use crate::{Options, OptionsName};

pub fn date_selector_onchange(use_state: &UseStateHandle<Option<NaiveDate>>) -> Callback<Event> {
    let use_state = use_state.clone();
    Callback::from(move |e: Event| {
        let result = HtmlInputElement::from(JsValue::from(e.target().unwrap().value_of())).value();
        use_state.set(Some(
            NaiveDate::parse_from_str(&result, "%Y-%m-%d").unwrap(),
        ));
    })
}

pub type Periode = [Option<NaiveDate>; 2];
#[derive(Clone, Copy)]
pub enum PeriodsIntervals {
    Start = 0,
    End = 1,
}

pub fn periode_selector_onchange(
    use_state: &UseStateHandle<Periode>,
    end_or_start: PeriodsIntervals,
) -> Callback<Event> {
    let use_state = use_state.clone();
    Callback::from(move |e: Event| {
        let result = HtmlInputElement::from(JsValue::from(e.target().unwrap().value_of())).value();
        let mut state = *use_state;
        state[end_or_start as usize] = if result.is_empty() {
            None
        } else {
            Some(NaiveDate::parse_from_str(&result, "%Y-%m-%d").unwrap())
        };
        use_state.set(state);
    })
}

pub fn number_selector_onchange(
    use_state: &UseStateHandle<i64>,
    multiplier: Option<i64>,
) -> Callback<Event> {
    let use_state = use_state.clone();
    Callback::from(move |e: Event| {
        let result = HtmlInputElement::from(JsValue::from(e.target().unwrap().value_of())).value();
        use_state.set(result.parse::<i64>().unwrap() * multiplier.unwrap_or(1));
    })
}

#[allow(dead_code)]
pub fn checkbox_selector_onchange(
    use_state: &UseStateHandle<Options>,
    option_name: OptionsName,
) -> Callback<Event> {
    let use_state = use_state.clone();
    Callback::from(move |e: Event| {
        let result =
            HtmlInputElement::from(JsValue::from(e.target().unwrap().value_of())).checked();
        let mut new_options = (*use_state).clone();
        match option_name {
            OptionsName::Crp => {
                new_options.crp = result;
            }
            OptionsName::Rps => {
                new_options.rps = result;
            }
        }
        use_state.set(new_options);
    })
}
