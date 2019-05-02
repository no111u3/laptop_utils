use sensors::{Sensors, Feature, Subfeature, SubfeatureType};

use std::rc::Rc;

#[derive(Debug)]
pub struct Thermal {
    pub cpu: Rc<Cpu>,
}

impl Thermal {
    pub fn new() -> Thermal {
        Thermal {
            cpu: Rc::new(Cpu::new(Sensors::new())),
        }
    }
}

#[derive(Debug)]
pub struct Cpu {
    pub package: Option<Package>,
    pub cores: Rc<Vec<Option<Core>>>,
}

impl Cpu {
    pub fn new(sensors: Sensors) -> Cpu {
        let mut package: Option<Package> = None;
        let mut cores: Vec<Option<Core>> = vec![];

        for chip in sensors {
            let chip_name = String::from(chip.get_name().unwrap());
            if chip_name.starts_with("coretemp") {
                for feature in chip {
                    let package_name = String::from(feature.get_label().unwrap());
                    let package_name: Vec<&str> = package_name.split_whitespace().collect();
                    match package_name[0] {
                        "Package" => {
                            package = Some(Package::new(feature));
                        }
                        "Core" => {
                            cores.push(Some(Core::new(feature, package_name[1])));
                        }
                        _ => {
                        }
                    }
                }
                break;
            }
        }
    
        Cpu {
            package: package,
            cores: Rc::new(cores),
        }
    }
}

#[derive(Debug)]
pub struct Package {
    pub current: Option<Subfeature>,
    pub max: Option<Subfeature>,
    pub crit: Option<Subfeature>,
    pub alarm: Option<Subfeature>,
}

impl Package {
    pub fn new(feature: Feature) -> Package {
        let (current, max, crit, alarm) = parse_subfeatures(feature);
    
        Package {
            current: current,
            max: max,
            crit: crit,
            alarm: alarm,
        }
    }
}

#[derive(Debug)]
pub struct Core {
    pub id: u8,
    pub current: Option<Subfeature>,
    pub max: Option<Subfeature>,
    pub crit: Option<Subfeature>,
    pub alarm: Option<Subfeature>,
}

impl Core {
    pub fn new(feature: Feature, core_id: &str) -> Core {
        let (current, max, crit, alarm) = parse_subfeatures(feature);

        Core {
            id: core_id.parse().unwrap(),
            current: current,
            max: max,
            crit: crit,
            alarm: alarm,
        }
    }
}

fn parse_subfeatures(feature: Feature)
    -> (Option<Subfeature>, Option<Subfeature>, Option<Subfeature>, Option<Subfeature>)
{
    (
        feature.get_subfeature(SubfeatureType::SENSORS_SUBFEATURE_TEMP_INPUT),
        feature.get_subfeature(SubfeatureType::SENSORS_SUBFEATURE_TEMP_MAX),
        feature.get_subfeature(SubfeatureType::SENSORS_SUBFEATURE_TEMP_CRIT),
        feature.get_subfeature(SubfeatureType::SENSORS_SUBFEATURE_TEMP_CRIT_ALARM),
    )
}

pub fn init() {
    let thermal = Thermal::new();
    
    let cores = thermal.cpu.cores.iter();
    
    for core in cores {
        println!("Core #{} - {}", core.as_ref().unwrap().id, core.as_ref().unwrap().current.as_ref().unwrap().get_value().unwrap());
    }

    let package = &thermal.cpu.clone().package;

    println!("Package - {}", package.as_ref().unwrap().current.as_ref().unwrap().get_value().unwrap());
}

#[cfg(test)]
mod tests {
}
