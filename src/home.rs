use chrono::NaiveDate;
use chronoutil::shift_years;
use yew::{function_component, html, use_state};

use crate::inputs::InputComponent;
use crate::{calculate, config::*, utils::*};

#[function_component(Home)]
pub fn home() -> Html {
    let incarceration_start_date = use_state(|| Some(NaiveDate::from_ymd(2022, 1, 1)));
    let selected_ppl = use_state(|| Some(DEFAULT_PPL));
    let substracted_crp = use_state(|| None);
    let manual_rps = use_state(|| None);
    let detention_period_1 = use_state(|| [None, None]);
    let detention_period_2 = use_state(|| [None, None]);

    let computed = calculate(
        (*incarceration_start_date).unwrap(),
        *selected_ppl,
        *detention_period_1,
        *detention_period_2,
        *substracted_crp,
        *manual_rps,
    );
    let substracted_crp_html_val = unwrap_or_empty_string(*substracted_crp);
    let manual_rps_html_val = unwrap_or_empty_string(*manual_rps);

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
                  value={Some(selected_ppl.unwrap_or_default().to_string())} onchange={number_selector_onchange(&selected_ppl, None)}  />
                <InputComponent itype="number" name="Années: "
                  value={Some((selected_ppl.unwrap_or_default() / ONE_YEAR).to_string())} onchange={number_selector_onchange(&selected_ppl, Some(ONE_YEAR))}  />
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
                  value={substracted_crp_html_val} onchange={number_selector_onchange(&substracted_crp, None)}  />
              </div>
            </div>
            <div class="table-row">
              <div class="table-data">{"RPS"}</div>
              <div class="table-data">
                <InputComponent itype="number" name="Mois donné: "
                  value={manual_rps_html_val} onchange={number_selector_onchange(&manual_rps, None)}  />
              </div>
            </div>
          </div>
        </div>
        <h1>{ "Résultats" }</h1>
        <div class="table">
          <div class="table-header">
            <div class="header__item">{""}</div>
            <div class="header__item">{"Aménagement probatoire"}</div>
            <div class="header__item">{"Aménagement de peine (mi-peine)"}</div>
            <div class="header__item">{"LSC"}</div>
            <div class="header__item">{"Fin de peine"}</div>
          </div>
          <div class="table-content">
            <div class="table-row center">
              <div class="table-data left">{"Date maximale"}</div>
              <div class="table-data">{shift_years(computed.mid_incarceration_end_date, -1).display_fr()}</div>
              <div class="table-data">{computed.mid_incarceration_end_date.display_fr()}</div>
              <div class="table-data">{"..."}</div>
              <div class="table-data">{computed.incarceration_end_date.display_fr()}</div>
            </div>
            <div class="table-row center">
              <div class="table-data left">{"Date prévisible (Déduction CRP et DP/ARSE)"}</div>
              <div class="table-data">{shift_years(computed.mid_incarceration_end_date_reducted_minus_rps, -1).display_fr()}</div>
              <div class="table-data">{computed.mid_incarceration_end_date_reducted_minus_rps.display_fr()}</div>
              <div class="table-data">{"..."}</div>
              <div class="table-data">{computed.incarceration_end_date_reducted_minus_rps.display_fr()}</div>
            </div>
            <div class="table-row center">
              <div class="table-data left">{"Date minimale (Déduction CRP, CP/ARSE, RPS)"}</div>
              <div class="table-data">{shift_years(computed.mid_incarceration_end_date_reducted, -1).display_fr()}</div>
              <div class="table-data">{computed.mid_incarceration_end_date_reducted.display_fr()}</div>
              <div class="table-data">{"..."}</div>
              <div class="table-data">{computed.incarceration_end_date_reducted.display_fr()}</div>
            </div>
          </div>
        </div>
        <h1>{ "Données avancées" }</h1>
        <div class="table">
          <div class="table-header">
            <div class="header__item">{""}</div>
            <div class="header__item">{"Mois"}</div>
            <div class="header__item">{"Jours"}</div>
          </div>
          <div class="table-content">
            <div class="table-row center">
              <div class="table-data left">{"Nombre de CRP prévisibles"}</div>
              <div class="table-data">{computed.previsional_crp_days / 30}</div>
              <div class="table-data">{computed.previsional_crp_days}</div>
              </div>
            <div class="table-row center">
              <div class="table-data left">{"Nombre de CRP calculés"}</div>
              <div class="table-data">{computed.previsional_crp_days / 30}</div>
              <div class="table-data">{computed.previsional_crp_days}</div>
              </div>
            <div class="table-row center">
              <div class="table-data left">{"Nombre de RPS prévisibles"}</div>
              <div class="table-data">{computed.previsional_rps_days / 30}</div>
              <div class="table-data">{computed.previsional_rps_days}</div>
              </div>
            <div class="table-row center">
              <div class="table-data left">{"Nombre de RPS"}</div>
              <div class="table-data">{computed.previsional_rps_days / 30}</div>
              <div class="table-data">{computed.previsional_rps_days}</div>
              </div>
            <div class="table-row center">
              <div class="table-data left">{"Durée de la DP/ARSE #1"}</div>
              <div class="table-data">{computed.days_detention_period_1 / 30}</div>
              <div class="table-data">{computed.days_detention_period_1}</div>
              </div>
            <div class="table-row center">
              <div class="table-data left">{"Durée de la DP/ARSE #2"}</div>
              <div class="table-data">{computed.days_detention_period_2 / 30}</div>
              <div class="table-data">{computed.days_detention_period_2}</div>
              </div>
            <div class="table-row center">
              <div class="table-data left">{"Total de réduction de peine (CRP/RPS)"}</div>
              <div class="table-data">{computed.total_crp_rps / 30}</div>
              <div class="table-data">{computed.total_crp_rps}</div>
              </div>
            <div class="table-row center">
              <div class="table-data left">{"Total de toutes les déductions (CRP/RPS/DP-ARSE)"}</div>
              <div class="table-data">{computed.total_crp_rps_detention / 30}</div>
              <div class="table-data">{computed.total_crp_rps_detention}</div>
              </div>
          </div>
        </div>
      </div>
    }
}
