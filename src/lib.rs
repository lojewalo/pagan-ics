use chrono::NaiveDate;
use equisol::SolarEvent;
use icalendar::{Calendar, Class, Component, Event};
use wasm_bindgen::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CrossQuarterDays {
    Imbolc,
    Beltane,
    Lammas,
    Samhain,
}

impl CrossQuarterDays {
    fn name(self) -> &'static str {
        match self {
            Self::Imbolc => "Imbolc",
            Self::Beltane => "Beltane",
            Self::Lammas => "Lammas",
            Self::Samhain => "Samhain",
        }
    }

    fn month(self) -> u32 {
        match self {
            Self::Imbolc => 2,
            Self::Beltane => 5,
            Self::Lammas => 8,
            Self::Samhain => 11,
        }
    }
}

#[wasm_bindgen]
pub struct CalendarOptions {
    #[wasm_bindgen(skip)]
    pub name: String,
    #[wasm_bindgen(skip)]
    pub description: Option<String>,

    pub year_start: i16,
    pub year_end: i16,

    #[wasm_bindgen(skip)]
    pub quarter_days: Vec<SolarEvent>,
    #[wasm_bindgen(skip)]
    pub quarter_days_fmt: String,

    #[wasm_bindgen(skip)]
    pub cross_quarter_days: Vec<CrossQuarterDays>,
    #[wasm_bindgen(skip)]
    pub cross_quarter_days_fmt: String,
}

#[wasm_bindgen]
impl CalendarOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str, start: i16, end: i16) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            year_start: start,
            year_end: end,
            quarter_days: vec![SolarEvent::MarchEquinox, SolarEvent::JuneSolstice, SolarEvent::SeptemberEquinox, SolarEvent::DecemberSolstice],
            quarter_days_fmt: "{pagan_name} ({name})".to_string(),
            cross_quarter_days: vec![CrossQuarterDays::Beltane, CrossQuarterDays::Imbolc, CrossQuarterDays::Lammas, CrossQuarterDays::Samhain],
            cross_quarter_days_fmt: "{pagan_name}".to_string(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    #[wasm_bindgen(getter)]
    pub fn quarter_days(&self) -> Vec<u8> {
        self.quarter_days
            .iter()
            .map(|event| match event {
                SolarEvent::MarchEquinox => 1,
                SolarEvent::JuneSolstice => 2,
                SolarEvent::SeptemberEquinox => 3,
                SolarEvent::DecemberSolstice => 4,
            })
            .collect()
    }

    #[wasm_bindgen(setter)]
    pub fn set_quarter_days(&mut self, quarter_days: Vec<u8>) {
        self.quarter_days = quarter_days
            .iter()
            .flat_map(|event| match event {
                1 => Some(SolarEvent::MarchEquinox),
                2 => Some(SolarEvent::JuneSolstice),
                3 => Some(SolarEvent::SeptemberEquinox),
                4 => Some(SolarEvent::DecemberSolstice),
                _ => None,
            })
            .collect();
    }

    #[wasm_bindgen(getter)]
    pub fn quarter_days_fmt(&self) -> String {
        self.quarter_days_fmt.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_quarter_days_fmt(&mut self, quarter_days_fmt: String) {
        self.quarter_days_fmt = quarter_days_fmt;
    }

    #[wasm_bindgen(getter)]
    pub fn cross_quarter_days(&self) -> Vec<u8> {
        self.cross_quarter_days
            .iter()
            .map(|event| match event {
                CrossQuarterDays::Imbolc => 1,
                CrossQuarterDays::Beltane => 2,
                CrossQuarterDays::Lammas => 3,
                CrossQuarterDays::Samhain => 4,
            })
            .collect()
    }

    #[wasm_bindgen(setter)]
    pub fn set_cross_quarter_days(&mut self, cross_quarter_days: Vec<u8>) {
        self.cross_quarter_days = cross_quarter_days
            .iter()
            .flat_map(|event| match event {
                1 => Some(CrossQuarterDays::Imbolc),
                2 => Some(CrossQuarterDays::Beltane),
                3 => Some(CrossQuarterDays::Lammas),
                4 => Some(CrossQuarterDays::Samhain),
                _ => None,
            })
            .collect();
    }

    #[wasm_bindgen(getter)]
    pub fn cross_quarter_days_fmt(&self) -> String {
        self.cross_quarter_days_fmt.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_cross_quarter_days_fmt(&mut self, cross_quarter_days_fmt: String) {
        self.cross_quarter_days_fmt = cross_quarter_days_fmt;
    }
}

fn make_calendar(opts: &CalendarOptions) -> Calendar {
    let mut cal = Calendar::new()
        .name(&opts.name)
        .done();

    if let Some(desc) = &opts.description {
        cal.description(desc);
    }

    for year in opts.year_start..=opts.year_end {
        for kind in &opts.quarter_days {
            if let Some(date) = kind.datetime(year) {
                let name = opts.quarter_days_fmt
                    .replace("{name}", kind.name())
                    .replace("{pagan_name}", kind.pagan_name());

                let event = Event::new()
                    .summary(&name)
                    .class(Class::Public)
                    .add_property("DTSTART", &date.format("%Y%m%dT%H%M%SZ").to_string())
                    .add_property("DTEND", &date.format("%Y%m%dT%H%M%SZ").to_string())
                    .done();
                cal.push(event);
            }
        }

        for kind in &opts.cross_quarter_days {
            let name = opts.cross_quarter_days_fmt
                .replace("{pagan_name}", kind.name());

            let date = NaiveDate::from_ymd(year as i32, kind.month(), 1);
            let event = Event::new()
                .summary(&name)
                .class(Class::Public)
                .all_day(date)
                .done();
            cal.push(event);
        }
    }

    cal
}

#[wasm_bindgen]
pub fn make_ics(opts: &CalendarOptions) -> String {
    make_calendar(opts).to_string()
}
