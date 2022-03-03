use std::ops::Sub;

use chrono::{Duration, NaiveDate};
use chronoutil::shift_months;

use crate::options::Options;

#[derive(Debug)]
pub struct Result {
    incarceration_end_data: NaiveDate,
    previsional_crp_days: i64,
    previsional_rps_days: i64,
    days_dp: i64,
    days_arse: i64,
    total_reduction_days: i64,
    mid_previsional_rps_days: i64,
    mid_total_reduction_days: i64,
    incarceration_end_data_reducted: NaiveDate,
    mid_incarceration_end_data: NaiveDate,
    mid_incarceration_end_data_reducted: NaiveDate,
}

impl Result {
    pub fn entries(&self) -> Vec<(&str, String)> {
        vec![
            (
                "Date de fin d'incarceration",
                self.incarceration_end_data.to_string(),
            ),
            ("CRP Prévisible", self.previsional_crp_days.to_string()),
            ("RPS Prévisible", self.previsional_rps_days.to_string()),
            ("Nombre de jours en DP", self.days_dp.to_string()),
            ("Nombre de jours en ARSE", self.days_arse.to_string()),
            (
                "Date de fin d'incarceration peine reduite",
                self.incarceration_end_data_reducted.to_string(),
            ),
            (
                "total_reduction_days",
                self.total_reduction_days.to_string(),
            ),
            ("Mi-Peine", self.mid_incarceration_end_data.to_string()),
            (
                "Mi-Peine reduite",
                self.mid_incarceration_end_data_reducted.to_string(),
            ),
            (
                "mid_previsional_rps_days",
                self.mid_previsional_rps_days.to_string(),
            ),
            (
                "mid_total_reduction_days",
                self.mid_total_reduction_days.to_string(),
            ),
        ]
    }
}

fn get_days_between_dates(end: Option<NaiveDate>, start: Option<NaiveDate>) -> i64 {
    if let (Some(end), Some(start)) = (end, start) {
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
const ONE_YEAR_CRP: i64 = 90;
const ONE_YEAR: i64 = 12;
const REDUCTION_PER_MONTH: i64 = 7;

pub fn calculate(
    incarceration_start_date: NaiveDate,
    month_ppl: i64,
    start_dp: Option<NaiveDate>,
    end_dp: Option<NaiveDate>,
    start_arse: Option<NaiveDate>,
    end_arse: Option<NaiveDate>,
    options: &Options,
) -> Result {
    let incarceration_end_data = shift_months(incarceration_start_date, month_ppl as i32);
    let mid_incarceration_end_data = shift_months(incarceration_start_date, month_ppl as i32 / 2);

    let previsional_crp_days: i64 = if options.crp {
        if month_ppl >= ONE_YEAR {
            let months_of_uncomplete_year = month_ppl % ONE_YEAR;
            let complete_years = month_ppl / ONE_YEAR;

            (complete_years * ONE_YEAR_CRP) + (months_of_uncomplete_year * REDUCTION_PER_MONTH)
        } else {
            month_ppl * REDUCTION_PER_MONTH
        }
    } else {
        0
    };
    let previsional_rps_days = if options.rps {
        let month_ppl_minus_crp = incarceration_end_data
            .sub(Duration::days(previsional_crp_days as i64))
            - incarceration_start_date;
        (month_ppl_minus_crp.num_days() / ONE_MONTH_DAYS) * REDUCTION_PER_MONTH
    } else {
        0
    };
    let days_dp = get_days_between_dates(end_dp, start_dp);
    let days_arse = get_days_between_dates(end_arse, start_arse);

    let (total_reduction_month, total_reduction_days) =
        split_months_days(previsional_crp_days + previsional_rps_days + days_dp + days_arse);
    let incarceration_end_data_reducted =
        shift_months(incarceration_end_data, -total_reduction_month as i32)
            .sub(Duration::days(total_reduction_days as i64));

    let mid_previsional_rps_days = if options.rps {
        let month_ppl_minus_crp = mid_incarceration_end_data
            .sub(Duration::days(previsional_crp_days / 2))
            - incarceration_start_date;
        (month_ppl_minus_crp.num_days() / ONE_MONTH_DAYS) * REDUCTION_PER_MONTH
    } else {
        0
    };
    let (mid_total_reduction_month, mid_total_reduction_days) = split_months_days(
        mid_previsional_rps_days + ((previsional_crp_days + days_dp + days_arse) / 2),
    );
    let mid_incarceration_end_data_reducted = shift_months(
        mid_incarceration_end_data,
        -mid_total_reduction_month as i32,
    )
    .sub(Duration::days(mid_total_reduction_days));
    Result {
        incarceration_end_data,
        previsional_crp_days,
        previsional_rps_days,
        days_dp,
        days_arse,
        total_reduction_days,
        incarceration_end_data_reducted,
        mid_total_reduction_days,
        mid_previsional_rps_days,
        mid_incarceration_end_data,
        mid_incarceration_end_data_reducted,
    }
}
