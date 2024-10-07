use std::collections::HashSet;

// first iteration:
// - unique: 604800
// - not unique: 35395200
// - percentage: 1.68 %

pub struct Timestamp {
    centiseconds: u32,
    seconds: u32,
    minutes: u32,
    hours: u32,
    set: HashSet<u32>,
}

impl Timestamp {
    pub fn new() -> Timestamp {
        Timestamp {
            centiseconds: 0,
            seconds: 0,
            minutes: 0,
            hours: 0,
            set: HashSet::new(),
        }
    }

    pub fn increment_with(&mut self, h: u32, m: u32, s: u32, c: u32) {
        let mut how_many_times_incr: u64 = 0;
        how_many_times_incr += c as u64;
        how_many_times_incr += s as u64 * 100;
        how_many_times_incr += m as u64 * 100 * 60;
        how_many_times_incr += h as u64 * 100 * 60 * 60;

        for i in 0..how_many_times_incr {
            self.increment();
        };
    }

    pub fn increment(&mut self) -> bool {

        // tf
        self.centiseconds += 1;
        if (self.centiseconds >= 100) {
            self.centiseconds = 0;
            self.seconds += 1;
            if (self.seconds >= 60) {
                self.seconds = 0;
                self.minutes += 1;
                if (self.minutes >= 60) {
                    println!("Hours: {}", self.hours);
                    self.minutes = 0;
                    self.hours += 1;
                    if (self.hours >= 100) {
                        return false;
                    }
                }
            }
        }


        true
    }

    pub fn print(&self) -> String {
        format!("{:02}:{:02}:{:02}:{:02}",
                self.hours,
                self.minutes,
                self.seconds,
                self.centiseconds)
    }

    pub fn is_unique(&mut self) -> bool {
        self.set.clear();

        let first_hours = self.hours / 10;
        let second_hours = self.hours - first_hours * 10;

        let first_minutes = self.minutes / 10;
        let second_minutes = self.minutes - first_minutes * 10;

        let first_seconds = self.seconds / 10;
        let second_seconds = self.seconds - first_seconds * 10;

        let first_centiseconds = self.centiseconds / 10;
        let second_centiseconds = self.centiseconds - first_centiseconds * 10;

        // self.set.insert(first_hours);
        // self.set.insert(second_hours);
        // self.set.insert(first_minutes);
        // self.set.insert(second_minutes);
        // self.set.insert(first_seconds);
        // self.set.insert(second_seconds);
        // self.set.insert(first_centiseconds);
        // self.set.insert(second_centiseconds);

        let unique_mask: u16 = (1 << first_hours) |
            (1 << second_hours) |
            (1 << first_minutes) |
            (1 << second_minutes) |
            (1 << first_seconds) |
            (1 << second_seconds) |
            (1 << first_centiseconds) |
            (1 << second_centiseconds);

        // Count bits set to 1 using built-in function
        let is_unique = unique_mask.count_ones() == 8;

        // println!("{}", set.len());
        //
        // println!("{},{}:{},{}:{},{}:{},{}",
        //          first_hours, second_hours,
        //          first_minutes, second_minutes,
        //          first_seconds, second_seconds,
        //          first_centiseconds, second_centiseconds
        // );

        self.set.len() == 8;
        is_unique

    }
}