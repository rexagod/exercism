#[derive(Debug, PartialEq)]
pub struct Clock{
    hours: i32,
    minutes:i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{hours: 0, minutes: 0}.add_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // convert everything to minutes within [0, 1339] (valid day minute)
        let new_mins = ((self.hours * 60 + self.minutes + minutes)%1440 + 1440)%1440;
        Clock{hours: new_mins/60, minutes: new_mins%60}
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}