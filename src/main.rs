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
    let substracted_crp = use_state(|| 0);
    let substracted_rps = use_state(|| 0);
    let detention_period_1 = use_state(|| [None, None]);
    let detention_period_2 = use_state(|| [None, None]);

    html! {
      <div class="App">
        <h1>{ "Calculateur de réductions de peine" }</h1>
        <span>{ "(Attention : Beta-testing)" }</span>
        <div class="table">
          <div class="table-header">
            <div class="header__item">{"Données"}</div>
          </div>
          <div class="table-content">
            <div class="table-row">
              <div class="table-data">{"Date d’incarcération"}</div>
              <div class="table-data">
                <InputComponent itype="date" value={Some((*incarceration_start_date).unwrap().to_string())} onchange={date_selector_onchange(&incarceration_start_date)}  />
              </div>
            </div>
            <div class="table-row">
              <div class="table-data">{"Durée de la peine"}</div>
              <div class="table-data">
                <InputComponent itype="number" name="Mois: "
                  value={Some((*selected_ppl).to_string())} onchange={number_selector_onchange(&selected_ppl, None)}  />
                <InputComponent itype="number" name="Années: "
                  value={Some((*selected_ppl / ONE_YEAR).to_string())} onchange={number_selector_onchange(&selected_ppl, Some(ONE_YEAR))}  />
              </div>
            </div>
            <div class="table-row">
              <div class="table-data">{"Détention provisoire / ARSE #1"}</div>
              <div class="table-data">
                <InputComponent itype="date" name="Début " onchange={periode_selector_onchange(&detention_period_1, PeriodsIntervals::Start)}  />
                <InputComponent itype="date" name="Fin " onchange={periode_selector_onchange(&detention_period_1, PeriodsIntervals::End)}  />
              </div>
            </div>
            <div class="table-row">
              <div class="table-data">{"Détention provisoire / ARSE #2"}</div>
              <div class="table-data">
                <InputComponent itype="date" name="Début " onchange={periode_selector_onchange(&detention_period_2, PeriodsIntervals::Start)}  />
                <InputComponent itype="date" name="Fin " onchange={periode_selector_onchange(&detention_period_2, PeriodsIntervals::End)}  />
              </div>
            </div>
            <div class="table-row">
              <div class="table-data">{"CRP"}</div>
              <div class="table-data">
                <InputComponent itype="number" name="Mois retiré: "
                  value={Some((*substracted_crp).to_string())} onchange={number_selector_onchange(&substracted_crp, None)}  />
              </div>
            </div>
            <div class="table-row">
              <div class="table-data">{"RPS"}</div>
              <div class="table-data">
                <InputComponent itype="number" name="Mois retiré: "
                  value={Some((*substracted_rps).to_string())} onchange={number_selector_onchange(&substracted_rps, None)}  />
              </div>
            </div>
          </div>
        </div>
        <h1>{ "Résultats" }</h1>
        <div class="table">
          <div class="table-header">
            <div class="header__item">{"Données"}</div>
          </div>
          <div class="table-content">
            {
              calculate((*incarceration_start_date).unwrap(), *selected_ppl,
                        *detention_period_1, *detention_period_2).entries().iter().map(|entrie| {
                          html!{
                            <div key={entrie.0} class="table-row">
                              <div class="table-data">{entrie.0}</div>
                              <div class="table-data">
                                <span>{&entrie.1}</span>
                              </div>
                            </div>
                          }
                        }).collect::<Html>()
            }
          </div>
        </div>
      </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
