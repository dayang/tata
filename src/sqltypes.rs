use chrono::NaiveDateTime;
use diesel::{Queryable, backend::Backend};

pub struct FormatedTime<T>(T);

impl<T: Into<String>> Into<String> for FormatedTime<T> {
    fn into(self) -> String {
        self.0.into()
    }
}

pub trait FromNaiveDateTime {
    fn from_naive_time(time: NaiveDateTime) -> Self;
}

impl<DB, ST, T> Queryable<ST, DB> for FormatedTime<T>
where
    DB: Backend,
    NaiveDateTime: Queryable<ST, DB>,
    T: FromNaiveDateTime
{
    type Row = <NaiveDateTime as Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> Self {
        FormatedTime(T::from_naive_time(NaiveDateTime::build(row)))
    }
}

pub struct HHMMDDDate(String);

impl FromNaiveDateTime for HHMMDDDate {
    fn from_naive_time(time: NaiveDateTime) -> Self {
        HHMMDDDate(time.format("%Y-%m-%d").to_string())
    }
}

impl Into<String> for HHMMDDDate{
    fn into(self) -> String {
        self.0
    }
}

pub struct HHMMDDHMTime(String);

impl FromNaiveDateTime for HHMMDDHMTime {
    fn from_naive_time(time: NaiveDateTime) -> Self {
        HHMMDDHMTime(time.format("%Y-%m-%d %H:%M").to_string())
    }
}

impl Into<String> for HHMMDDHMTime{
    fn into(self) -> String {
        self.0
    }
}