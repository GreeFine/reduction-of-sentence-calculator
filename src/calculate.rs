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
            ("Mi-Peine", self.mid_incarceration_end_data.to_string()),
            (
                "Mi-Peine reduite",
                self.mid_incarceration_end_data_reducted.to_string(),
            ),
            (
                "total_reduction_days",
                self.total_reduction_days.to_string(),
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

    let previsional_crp_days: i64 = if options.crp {
        if month_ppl >= 12 {
            let months_of_uncomplete_year = month_ppl % 12;
            // FIXME: 60 => 2 month? check with real dates
            let caped_months_of_uncomplete_year = if (months_of_uncomplete_year * 7) >= 60 {
                60
            } else {
                months_of_uncomplete_year
            };
            caped_months_of_uncomplete_year + ((month_ppl - months_of_uncomplete_year) * 7)
        } else {
            month_ppl * 7
        }
    } else {
        0
    };
    let previsional_rps_days = if options.rps {
        // TODO: Manque les CRP previsible ?
        let month_ppl_minus_crp = incarceration_end_data
            .sub(Duration::days(previsional_crp_days as i64))
            - incarceration_start_date;
        // FIXME: here using 30 days for a month
        (month_ppl_minus_crp.num_days() / 30) * 7
    } else {
        0
    };
    let days_dp = get_days_between_dates(end_dp, start_dp);
    let days_arse = get_days_between_dates(end_arse, start_arse);

    let total_reduction_days = previsional_crp_days + previsional_rps_days + days_dp + days_arse;
    let incarceration_end_data_reducted =
        incarceration_end_data.sub(Duration::days(total_reduction_days as i64));

    let mid_incarceration_end_data = shift_months(incarceration_start_date, month_ppl as i32 / 2);

    let mid_incarceration_end_data_reducted =
        mid_incarceration_end_data.sub(Duration::days(total_reduction_days / 2));
    Result {
        incarceration_end_data,
        previsional_crp_days,
        previsional_rps_days,
        days_dp,
        days_arse,
        total_reduction_days,
        incarceration_end_data_reducted,
        mid_incarceration_end_data,
        mid_incarceration_end_data_reducted,
    }
}
