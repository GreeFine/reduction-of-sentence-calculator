mod calculate;
mod inputs;
mod options;
mod utils;

use calculate::calculate;
use chrono::NaiveDate;
use inputs::InputComponent;
use utils::*;
use yew::prelude::*;

use crate::options::{Options, OptionsName};

const DEFAULT_PPL: i64 = 24;
const ONE_YEAR: i64 = 12;
#[function_component(App)]
fn app() -> Html {
    let incarceration_start_date = use_state(|| Some(NaiveDate::from_ymd(2022, 1, 1)));
    let selected_ppl = use_state(|| DEFAULT_PPL);
    let start_dp = use_state(|| None);
    let end_dp = use_state(|| None);
    let start_arse = use_state(|| None);
    let end_arse = use_state(|| None);
    let options = use_state(Options::default);

    html! {
      <div class="App">
        <h1>{ "Calculateur" }</h1>
        <div class="flex-row">
          <div>
            <InputComponent itype="date" name="Debut incarceration: " value={Some((*incarceration_start_date).unwrap().to_string())} onchange={date_selector_onchange(&incarceration_start_date)}  />
            <div>
              <span>{"Durée PPL"}</span>
              <InputComponent itype="number" name="Mois: "
                  value={Some((*selected_ppl).to_string())} onchange={number_selector_onchange(&selected_ppl, None)}  />
              <InputComponent itype="number" name="Années: "
                  value={Some((*selected_ppl / ONE_YEAR).to_string())} onchange={number_selector_onchange(&selected_ppl, Some(ONE_YEAR))}  />
            </div>
          </div>
          <div>
            <div>
              <InputComponent itype="number" name="CRP : "
                  value={Some((*selected_ppl / ONE_YEAR).to_string())} onchange={number_selector_onchange(&selected_ppl, Some(24))}  />
              <InputComponent itype="checkbox" checked={options.crp} name="CRP previsible: "
                  onchange={checkbox_selector_onchange(&options, OptionsName::Crp)}  />
            </div>
            <div>
              <InputComponent itype="number" name="RPS: "
                  value={Some((*selected_ppl / ONE_YEAR).to_string())} onchange={number_selector_onchange(&selected_ppl, Some(24))}  />
              <InputComponent itype="checkbox" checked={options.rps} name="RPS previsible: "
                  onchange={checkbox_selector_onchange(&options, OptionsName::Rps)}  />
            </div>
          </div>
          <div>
            <InputComponent itype="date" name="Debut Detention Provisoire: " onchange={date_selector_onchange(&start_dp)}  />
            <InputComponent itype="date" name="Fin Detention Provisoire: " onchange={date_selector_onchange(&end_dp)}  />
          </div>
          <div>
            <InputComponent itype="date" name="Debut ARSE: " onchange={date_selector_onchange(&start_arse)}  />
            <InputComponent itype="date" name="Fin ARSE: " onchange={date_selector_onchange(&end_arse)}  />
          </div>
        </div>
        <h1>{ "Resultat" }</h1>
        <div class="flex-row flex-wrap">
          {
            calculate((*incarceration_start_date).unwrap(), *selected_ppl,
                      *start_dp, *end_dp,
                      *start_arse, *end_arse, &options).entries().iter().map(|entrie| {
                        html!{
                          <div key={entrie.0}>
                            <br />
                            <h3>{entrie.0}</h3>
                            <span>{&entrie.1}</span>
                          </div>
                        }
                      }).collect::<Html>()
          }
        </div>
      </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
