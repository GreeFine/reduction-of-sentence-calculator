mod calculate;
mod inputs;
mod utils;

use calculate::calculate;
use chrono::NaiveDate;
use inputs::InputComponent;
use utils::*;
use yew::prelude::*;

const DEFAULT_PPL: usize = 24;

#[function_component(App)]
fn app() -> Html {
    let incarceration_start_date = use_state(|| Some(NaiveDate::from_ymd(2022, 1, 1)));
    let selected_ppl = use_state(|| DEFAULT_PPL);
    let start_dp = use_state(|| None);
    let end_dp = use_state(|| None);
    let start_arse = use_state(|| None);
    let end_arse = use_state(|| None);

    html! {
        <>
            <h1>{ "Calculator" }</h1>
            <div>
                <InputComponent itype="date" name="Debut incarceration: " onchange={date_selector_onchange(&incarceration_start_date)}  />
                <InputComponent itype="number" name="Mois PPL: " onchange={number_selector_onchange(&selected_ppl)}  />
                <InputComponent itype="date" name="Debut Detention Provisoire: " onchange={date_selector_onchange(&start_dp)}  />
                <InputComponent itype="date" name="Fin Detention Provisoire: " onchange={date_selector_onchange(&end_dp)}  />
                <InputComponent itype="date" name="Debut ARSE: " onchange={date_selector_onchange(&start_arse)}  />
                <InputComponent itype="date" name="Fin ARSE: " onchange={date_selector_onchange(&end_arse)}  />
                <h2>{ "Result" }</h2>
                <div>
                  {
                    calculate((*incarceration_start_date).unwrap(), *selected_ppl,
                              *start_dp, *end_dp,
                              *start_arse, *end_arse).entries().iter().map(|entrie| {
                                html!{
                                  <>
                                    <br />
                                    <span key={entrie.0}>{ format!("{} => {}", entrie.0, entrie.1) }</span>
                                  </>
                                }
                              }).collect::<Html>()
                  }
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
