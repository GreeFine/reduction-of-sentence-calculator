mod utils;

use serde::{Deserialize, Serialize};
use std::{cmp::min, ops::Sub};
use wasm_bindgen::prelude::*;

use chrono::{Duration, NaiveDate};
use chronoutil::shift_months;

use crate::utils::{year_months_from_months, Periode, PeriodsIntervals};

#[derive(Debug, Serialize)]
pub struct Result {
    pub incarceration_end_date: NaiveDate,
    pub previsional_crp_days: i64,
    pub previsional_rps_days: i64,
    pub total_detention_days: i64,
    pub total_reduction_days: i64,
    pub mid_previsional_rps_days: i64,
    pub mid_total_reduction_days: i64,
    pub incarceration_end_date_reducted: NaiveDate,
    pub incarceration_end_date_reducted_minus_rps: NaiveDate,
    pub mid_incarceration_end_date: NaiveDate,
    pub mid_incarceration_end_date_reducted_minus_rps: NaiveDate,
    pub mid_incarceration_end_date_reducted: NaiveDate,
    pub days_detention_period_1: i64,
    pub days_detention_period_2: i64,
    pub total_crp_rps: i64,
    pub total_crp_rps_detention: i64,
}

fn get_days_between_dates(periode: Periode) -> i64 {
    if let (Some(start), Some(end)) = (
        periode[PeriodsIntervals::Start as usize],
        periode[PeriodsIntervals::End as usize],
    ) {
        if end > start {
            (end - start).num_days()
        } else {
            0
        }
    } else {
        0
    }
}

fn split_months_days(days: i64) -> (i64, i64) {
    let months = days / ONE_MONTH_DAYS;
    (months, days - (months * ONE_MONTH_DAYS))
}

// FIXME: 30 => 1 month? check with real dates
const ONE_MONTH_DAYS: i64 = 30;
const ONE_YEAR_CRP: i64 = 60;
const FIRST_YEAR_CRP: i64 = 90;
const YEARS_RPS: i64 = 90;
const ONE_YEAR: i64 = 12;
const REDUCTION_PER_MONTH: i64 = 7;

#[derive(Debug, Deserialize)]
struct Inputs {
    incarceration_start_date: NaiveDate,
    month_ppl: Option<i64>,
    detention_period_1: Periode,
    detention_period_2: Periode,
    substracted_crp: Option<i64>,
    manual_rps: Option<i64>,
}

#[wasm_bindgen]
pub fn calculate(input: String) -> String {
    let inputs: Inputs = serde_json::from_str(&input).unwrap();

    let month_ppl = inputs.month_ppl.unwrap_or_default();
    let incarceration_end_date = shift_months(inputs.incarceration_start_date, month_ppl as i32);
    let mid_incarceration_end_date =
        shift_months(inputs.incarceration_start_date, month_ppl as i32 / 2);

    let mut previsional_crp_days: i64 = if month_ppl >= ONE_YEAR {
        let (months_of_uncomplete_year, complete_years) = year_months_from_months(month_ppl);

        FIRST_YEAR_CRP
            + ((complete_years - 1) * ONE_YEAR_CRP)
            + (min(
                months_of_uncomplete_year * REDUCTION_PER_MONTH,
                ONE_YEAR_CRP,
            ))
    } else {
        month_ppl * REDUCTION_PER_MONTH
    };
    previsional_crp_days -= inputs.substracted_crp.unwrap_or_default() * ONE_MONTH_DAYS;

    let (splited_previsional_crp_month, splited_previsional_crp_days) =
        split_months_days(previsional_crp_days);
    let month_ppl_minus_crp = shift_months(
        incarceration_end_date,
        -splited_previsional_crp_month as i32,
    )
    .sub(Duration::days(splited_previsional_crp_days as i64))
        - inputs.incarceration_start_date;
    let previsional_rps_days = if let Some(month) = inputs.manual_rps {
        month * ONE_MONTH_DAYS
    } else {
        let (months_of_uncomplete_year, complete_years) =
            year_months_from_months(month_ppl_minus_crp.num_days() / ONE_MONTH_DAYS);

        (months_of_uncomplete_year * REDUCTION_PER_MONTH) + (complete_years * YEARS_RPS)
    };

    let days_detention_period_1 = get_days_between_dates(inputs.detention_period_1);
    let days_detention_period_2 = get_days_between_dates(inputs.detention_period_2);
    let total_detention_days = days_detention_period_1 + days_detention_period_2;

    let (total_reduction_month, total_reduction_days) =
        split_months_days(previsional_crp_days + total_detention_days);
    let incarceration_end_date_reducted_minus_rps =
        shift_months(incarceration_end_date, -total_reduction_month as i32)
            .sub(Duration::days(total_reduction_days as i64));

    let (total_reduction_month, total_reduction_days) =
        split_months_days(previsional_crp_days + previsional_rps_days + total_detention_days);
    let incarceration_end_date_reducted =
        shift_months(incarceration_end_date, -total_reduction_month as i32)
            .sub(Duration::days(total_reduction_days as i64));

    let month_ppl_minus_crp = mid_incarceration_end_date
        .sub(Duration::days(previsional_crp_days / 2))
        - inputs.incarceration_start_date;
    let mid_previsional_rps_days =
        (month_ppl_minus_crp.num_days() / ONE_MONTH_DAYS) * REDUCTION_PER_MONTH;

    let (mid_total_reduction_month, mid_total_reduction_days) =
        split_months_days(mid_previsional_rps_days + (total_detention_days / 2));
    let mid_incarceration_end_date_reducted_minus_rps = shift_months(
        mid_incarceration_end_date,
        -mid_total_reduction_month as i32,
    )
    .sub(Duration::days(mid_total_reduction_days));

    let (mid_total_reduction_month, mid_total_reduction_days) = split_months_days(
        mid_previsional_rps_days + ((previsional_crp_days + total_detention_days) / 2),
    );
    let mid_incarceration_end_date_reducted = shift_months(
        mid_incarceration_end_date,
        -mid_total_reduction_month as i32,
    )
    .sub(Duration::days(mid_total_reduction_days));

    serde_json::to_string(&Result {
        incarceration_end_date,
        previsional_crp_days,
        previsional_rps_days,
        total_detention_days,
        total_reduction_days,
        incarceration_end_date_reducted,
        incarceration_end_date_reducted_minus_rps,
        mid_total_reduction_days,
        mid_previsional_rps_days,
        mid_incarceration_end_date,
        mid_incarceration_end_date_reducted_minus_rps,
        mid_incarceration_end_date_reducted,
        days_detention_period_1,
        days_detention_period_2,
        total_crp_rps: previsional_crp_days + previsional_rps_days,
        total_crp_rps_detention: previsional_crp_days + previsional_rps_days + total_detention_days,
    })
    .unwrap()
}
