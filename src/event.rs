use std::cmp::Ordering;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, Default)]
pub struct Event {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub message: String
}

impl Event {
    pub fn from_string(date: &str, message: &str) -> Result<Self, String> {
        let splited: Vec<&str> = date.split('-').collect();    
        let date_infos: Vec<&str> = splited[0].split('/').collect();

        if date_infos.len() != 3 {
            return Err("Missing year, month or day".to_owned());
        }

        let year = match date_infos[0].parse::<u16>() {
            Ok(x) => x,
            Err(_) => return Err("Invalid year".to_owned())
        };

        let date_infos: Vec<Option<u8>> = date_infos.iter()
            .skip(1)
            .map(|x| match x.parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None
            })
            .collect();

        if date_infos.contains(&None) {
            return Err("Invalid month or day".to_owned());
        }

        let date_infos: Vec<u8> = date_infos.iter()
            .map(|x| x.unwrap())
            .collect();

        let mut event = Self {
            year,
            month: date_infos[0],
            day: date_infos[1],
            ..Default::default()
        };

        if let Some(date_details) = splited.get(1) {
            let date_details: Vec<&str> = date_details.split(':').collect();

            if date_details.len() > 3 {
                return Err("Too much date details were given".to_owned());
            }

            let date_details: Vec<Option<u8>> = date_details.iter()
                .map(|x| match x.parse::<u8>() {
                    Ok(x) => Some(x),
                    Err(_) => None
                })
                .collect();
        
            if date_details.contains(&None) {
                return Err("Invalid hour, minute or second".to_owned());
            }

            let date_details: Vec<u8> = date_details.iter()
                .map(|x| x.unwrap())
                .collect();
            
            if let Some(hour) = date_details.get(0) {
                event.hour = *hour;
                
                if let Some(minute) = date_details.get(1) {
                    event.minute = *minute;

                    if let Some(second) = date_details.get(2) {
                        event.second = *second;
                    }
                }
            }
        }

        event.message = message.to_owned();

        Ok(event)
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut result = self.year.cmp(&other.year);

        match result {
            Ordering::Equal => (),
            _ => return result
        }

        let cmp_array = [
            (&self.month, &other.month),
            (&self.day, &other.day),
            (&self.hour, &other.hour),
            (&self.minute, &other.minute),
            (&self.second, &other.second)
        ];

        for (first, second) in cmp_array {
            result = first.cmp(second);

            match result {
                Ordering::Equal => (),
                _ => return result
            }
        }

        result
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}
