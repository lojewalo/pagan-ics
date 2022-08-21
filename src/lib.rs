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

    #[wasm_bindgen(skip)]
    pub custom_names: Vec<Option<String>>,
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
            custom_names: vec![None, None, None, None, None, None, None, None],
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
            .map(|event| event.index() as u8)
            .collect()
    }

    #[wasm_bindgen(setter)]
    pub fn set_quarter_days(&mut self, quarter_days: Vec<u8>) {
        self.quarter_days = quarter_days
            .iter()
            .flat_map(|&event| SolarEvent::from_index(event as usize))
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
            .map(|event| event.index() as u8)
            .collect()
    }

    #[wasm_bindgen(setter)]
    pub fn set_cross_quarter_days(&mut self, cross_quarter_days: Vec<u8>) {
        self.cross_quarter_days = cross_quarter_days
            .iter()
            .flat_map(|&event| CrossQuarterDays::from_index(event as usize))
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

    #[wasm_bindgen(getter)]
    pub fn custom_names(&self) -> Box<[JsValue]> {
        self.custom_names
            .iter()
            .cloned()
            .map(Into::into)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    #[wasm_bindgen(setter)]
    pub fn set_custom_names(&mut self, custom_names: Box<[JsValue]>) {
        self.custom_names = custom_names
            .into_iter()
            .map(|x| x.as_string())
            .collect();
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
                let pagan_name = opts.custom_names.get(kind.index())
                    .map(|x| x.as_deref())
                    .flatten()
                    .unwrap_or_else(|| kind.pagan_name());

                let name = opts.quarter_days_fmt
                    .replace("{name}", kind.name())
                    .replace("{pagan_name}", pagan_name);

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
            let pagan_name = opts.custom_names.get(4 + kind.index())
                .map(|x| x.as_deref())
                .flatten()
                .unwrap_or_else(|| kind.name());

            let name = opts.cross_quarter_days_fmt
                .replace("{pagan_name}", pagan_name);

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

trait CustomNameIndex : Sized {
    fn index(self) -> usize;

    fn from_index(index: usize) -> Option<Self>;
}

impl CustomNameIndex for SolarEvent {
    fn index(self) -> usize {
        match self {
            SolarEvent::MarchEquinox => 0,
            SolarEvent::JuneSolstice => 1,
            SolarEvent::SeptemberEquinox => 2,
            SolarEvent::DecemberSolstice => 3,
        }
    }

    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(SolarEvent::MarchEquinox),
            1 => Some(SolarEvent::JuneSolstice),
            2 => Some(SolarEvent::SeptemberEquinox),
            3 => Some(SolarEvent::DecemberSolstice),
            _ => None,
        }
    }
}

impl CustomNameIndex for CrossQuarterDays {
    fn index(self) -> usize {
        match self {
            CrossQuarterDays::Imbolc => 0,
            CrossQuarterDays::Beltane => 1,
            CrossQuarterDays::Lammas => 2,
            CrossQuarterDays::Samhain => 3,
        }
    }

    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(CrossQuarterDays::Imbolc),
            1 => Some(CrossQuarterDays::Beltane),
            2 => Some(CrossQuarterDays::Lammas),
            3 => Some(CrossQuarterDays::Samhain),
            _ => None,
        }
    }
}
