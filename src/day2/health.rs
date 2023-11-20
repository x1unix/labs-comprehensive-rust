// day2-morning-ep2
// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: usize,
    last_blood_pressure: Option<(u32, u32)>,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        // Create a new User instance
        Self {
            name,
            age,
            height,
            visit_count: 0,
            last_blood_pressure: None,
        }
    }

    pub fn name(&self) -> &str {
        // Return the user's name
        &self.name
    }

    pub fn age(&self) -> u32 {
        // Return the user's age
        self.age
    }

    pub fn height(&self) -> f32 {
        // Return the user's height
        self.height
    }

    pub fn doctor_visits(&self) -> u32 {
        // Return the number of time the user has visited the doctor
        self.visit_count as u32
    }

    pub fn set_age(&mut self, new_age: u32) {
        // Set the user's age
        self.age = new_age
    }

    pub fn set_height(&mut self, new_height: f32) {
        // Set the user's height
        self.height = new_height
    }

    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        // Update a user's statistics based on measurements from a visit to the doctor
        let Measurements {
            height,
            blood_pressure,
        } = measurements;

        self.visit_count += 1;
        let result = HealthReport {
            patient_name: &self.name,
            visit_count: self.visit_count as u32,
            height_change: height - self.height,
            blood_pressure_change: match self.last_blood_pressure {
                Some(prev) => diff_pressure(prev, blood_pressure),
                None => None,
            },
        };

        self.height = measurements.height;
        self.last_blood_pressure = Some(measurements.blood_pressure);
        result
    }
}

fn vec2_utoi(v: (u32, u32)) -> (i32, i32) {
    (v.0 as i32, v.1 as i32)
}

fn diff_pressure(prev: (u32, u32), cur: (u32, u32)) -> Option<(i32, i32)> {
    if prev == cur {
        return None;
    }

    let (prev_x, prev_y) = vec2_utoi(prev);
    let (cur_x, cur_y) = vec2_utoi(cur);

    Some((cur_x - prev_x, cur_y - prev_y))
}

#[test]
fn test_main() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name(), bob.age());
}

#[test]
fn test_height() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.height(), 155.2);
}

#[test]
fn test_set_age() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.age(), 32);
    bob.set_age(33);
    assert_eq!(bob.age(), 33);
}

#[test]
fn test_visit() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.doctor_visits(), 0);
    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (120, 80),
    });
    assert_eq!(report.patient_name, "Bob");
    assert_eq!(report.visit_count, 1);
    assert_eq!(report.blood_pressure_change, None);

    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (115, 76),
    });

    assert_eq!(report.visit_count, 2);
    assert_eq!(report.blood_pressure_change, Some((-5, -4)));
}
