use chrono::NaiveDate;

pub const ONE_YEAR: i64 = 12;

pub type Periode = [Option<NaiveDate>; 2];
#[derive(Clone, Copy)]
pub enum PeriodsIntervals {
    Start = 0,
    End = 1,
}

pub fn year_months_from_months(months: i64) -> (i64, i64) {
    (months % ONE_YEAR, months / ONE_YEAR)
}
