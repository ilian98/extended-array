/// This file and package manages the demo - downloads csv data for the coronavirus, parses it and returns it to main
use std::{collections::HashMap, str::FromStr};

use reqwest::Error;
/// Element is a struct used for storing the important information in one line of the csv data
pub struct Element {
    /// name of the country, we make all names in lowercase and also all spaces are turned to underscore
    country: String,
    /// string in the format yyyy-ww
    year_week: String,
    /// number of total cases for that week of the year
    pub cases: u64,
    /// number of total deaths for that week of the year
    pub deaths: u64,
}

use std::fmt::{self, Display, Formatter};
impl Display for Element {
    /// Element is displayed with the fields separated by a semicolon with no spaces around the fields
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{};{};{};{};",
            self.country, self.year_week, self.cases, self.deaths
        )
    }
}

impl Clone for Element {
    fn clone(&self) -> Self {
        Element {
            country: self.country.clone(),
            deaths: self.deaths,
            cases: self.cases,
            year_week: self.year_week.clone(),
        }
    }
}

impl FromStr for Element {
    type Err = String;
    /// from_str expects the same format as the output of the Display trait
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(';').map(|s| s.trim()).collect();

        if parts.len() != 4 {
            return Err(String::from("Wrong number of parts"));
        }
        let country = parts[0];
        let year_week = parts[1];
        if year_week.len() != 7 {
            return Err(String::from("Wrong len of year week"));
        }
        let cases = match parts[2].parse::<u64>() {
            Err(e) => return Err(e.to_string()),
            Ok(n) => n,
        };
        let deaths = match parts[3].parse::<u64>() {
            Err(e) => return Err(e.to_string()),
            Ok(n) => n,
        };
        Ok(Element {
            country: country.to_string().to_lowercase(),
            year_week: year_week.to_string(),
            cases: cases,
            deaths: deaths,
        })
    }
}

use crate::exray::Exray;

use super::csv_parser::*;

#[derive(Debug)]
pub enum DemoError {
    DataLoadError(Error),
    CsvError(CsvError),
}
/// this is the main function which uses reqwest::blocking to download the csv data as string, then the csv_parser to parse it and in the end return either Vector with the data or DemoError
pub fn get_data() -> Result<Vec<Element>, DemoError> {
    println!("Starting to load coronavirus data.");
    let data = match reqwest::blocking::get(
        "https://opendata.ecdc.europa.eu/covid19/nationalcasedeath/csv",
    ) {
        Err(e) => return Err(DemoError::DataLoadError(e)),
        Ok(r) => match r.text() {
            Err(e) => return Err(DemoError::DataLoadError(e)),
            Ok(s) => s,
        },
    };
    println!("Data loaded.");

    let mut lines = data.lines();
    // we make new csv that parses the first row which is the header with the columns' names
    let mut csv = match Csv::new(lines.next().unwrap().as_bytes()) {
        Err(e) => return Err(DemoError::CsvError(e)),
        Ok(c) => c,
    };
    let mut result = Vec::<Element>::new(); // the result that is returned in the end
    let mut cases = HashMap::<(String, String), usize>::new(); // this is used to strore the index of country&year_week in the Vector
    while let Some(line) = lines.next() {
        match csv.parse_line(line) {
            Err(e) => println!("Error when parsing {} - {:?}", line, e),
            Ok(r) => {
                let country = r
                    .get("\u{feff}country") // for some reason the first field which is country is parsed with starting \u{feff}
                    .unwrap()
                    .to_lowercase()
                    .replace(' ', "_");
                let year_week = r.get("year_week").unwrap();

                let mut count = match r.get("weekly_count").unwrap().parse::<i64>() {
                    Err(e) => {
                        println!(
                            "Error when parsing {} - {:?}",
                            r.get("weekly_count").unwrap(),
                            e
                        );
                        continue;
                    }
                    Ok(num) => num,
                };
                if count < 0 {
                    // aparently there are some rows with negative count for cases or deaths?!
                    count *= -1;
                }

                if r.get("indicator").unwrap() == "cases" {
                    let e = Element {
                        country: country.clone(),
                        year_week: year_week.clone(),
                        cases: count as u64,
                        deaths: 0,
                    };
                    result.push(e);
                    cases.insert((country.clone(), year_week.clone()), result.len() - 1);
                } else {
                    // the format of the csv is the following - first the rows with the number of cases then the rows with the number of deaths
                    let ind = cases.get(&(country.clone(), year_week.clone())).unwrap();
                    result[*ind].deaths = count as u64;
                }
            }
        }
    }
    println!("Data parsed.");

    return Ok(result);
}

// this functions is used by main to tell the segment where the country data is in the exray, we use binary searches on the country name because the data in the csv is sorted in ascending order of the country name and year_week
pub fn find_country_segment(
    mut country: String,
    exray: &Exray<Element, (f64, f64)>,
) -> Option<(usize, usize)> {
    country = country.to_lowercase();
    let mut l = -1;
    let mut r = exray.len() as i32;
    let mut mid;
    // binary search for the beginning of the segment
    while l < r - 1 {
        mid = (l + r) / 2;
        if exray[mid as usize].country >= country {
            r = mid;
        } else {
            l = mid;
        }
    }
    if r == exray.len() as i32 || exray[r as usize].country != country {
        return None;
    }
    let beg_ind = r;

    // binary search for the end of the segment
    l = -1;
    r = exray.len() as i32;
    while l < r - 1 {
        mid = (l + r) / 2;
        if exray[mid as usize].country <= country {
            l = mid;
        } else {
            r = mid;
        }
    }
    let end_ind = l;

    return Some((beg_ind as usize, end_ind as usize));
}
