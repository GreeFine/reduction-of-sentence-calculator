use std::ops::Add;

use chrono::{Duration, NaiveDate};
use chronoutil::shift_months;

#[derive(Debug)]
pub struct Result {
    incarceration_end_data: NaiveDate,
    previsional_crp: usize,
    previsional_rps: usize,
    days_dp: usize,
    days_arse: usize,
    total_reduction_months: usize,
    total_reduction_days: usize,
    incarceration_end_data_reducted: NaiveDate,
    mid_incarceration_end_data: NaiveDate,
    mid_incarceration_end_data_reducted: NaiveDate,
}

impl Result {
    pub fn entries(&self) -> Vec<(&str, String)> {
        vec![
            (
                "incarceration_end_data",
                self.incarceration_end_data.to_string(),
            ),
            ("previsional_crp", self.previsional_crp.to_string()),
            ("previsional_rps", self.previsional_rps.to_string()),
            ("days_dp", self.days_dp.to_string()),
            ("days_arse", self.days_arse.to_string()),
            (
                "total_reduction_months",
                self.total_reduction_months.to_string(),
            ),
            (
                "total_reduction_days",
                self.total_reduction_days.to_string(),
            ),
            (
                "incarceration_end_data_reducted",
                self.incarceration_end_data_reducted.to_string(),
            ),
            (
                "mid_incarceration_end_data",
                self.mid_incarceration_end_data.to_string(),
            ),
            (
                "mid_incarceration_end_data_reducted",
                self.mid_incarceration_end_data_reducted.to_string(),
            ),
        ]
    }
}

pub fn calculate(
    incarceration_start_date: NaiveDate,
    month_ppl: usize,
    start_dp: Option<NaiveDate>,
    end_dp: Option<NaiveDate>,
    start_arse: Option<NaiveDate>,
    end_arse: Option<NaiveDate>,
) -> Result {
    let incarceration_end_data = shift_months(incarceration_start_date, month_ppl as i32);
    let previsional_crp = 1 + 2 * (month_ppl / 12);
    let previsional_rps = 3 * (month_ppl / 12);
    let days_dp = if let (Some(end_dp), Some(start_dp)) = (end_dp, start_dp) {
        (end_dp - start_dp).num_days()
    } else {
        0
    };
    let days_arse = if let (Some(end_arse), Some(start_arse)) = (end_arse, start_arse) {
        (end_arse - start_arse).num_days()
    } else {
        0
    };
    let total_reduction_months = previsional_crp + previsional_rps;
    let total_reduction_days = days_dp + days_arse;
    let incarceration_end_data_reducted =
        shift_months(incarceration_end_data, -(total_reduction_months as i32))
            .add(Duration::days(total_reduction_days));

    let mid_incarceration_end_data = shift_months(incarceration_start_date, month_ppl as i32 / 2);

    let mid_incarceration_end_data_reducted = shift_months(
        mid_incarceration_end_data,
        -((total_reduction_months / 2) as i32),
    )
    .add(Duration::days(total_reduction_days / 2));
    Result {
        incarceration_end_data,
        previsional_crp,
        previsional_rps,
        days_dp: days_dp as usize,
        days_arse: days_arse as usize,
        total_reduction_months,
        total_reduction_days: total_reduction_days as usize,
        incarceration_end_data_reducted,
        mid_incarceration_end_data,
        mid_incarceration_end_data_reducted,
    }
}
