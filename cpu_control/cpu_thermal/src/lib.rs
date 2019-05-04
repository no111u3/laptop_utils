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
    package: Option<Package>,
    cores: Rc<Vec<Option<Core>>>,
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

    pub fn package(&self) -> Option<&Package> {
        self.package.as_ref()
    }

    pub fn cores(&self) -> Vec<Option<&Core>> {
        self.cores.iter().map(|x| x.as_ref()).collect()
    }
}

#[derive(Debug)]
pub struct Package {
    current: Option<Subfeature>,
    max: Option<Subfeature>,
    crit: Option<Subfeature>,
    alarm: Option<Subfeature>,
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

    pub fn current(&self) -> Option<&Subfeature> {
        self.current.as_ref()
    }

    pub fn max(&self) -> Option<&Subfeature> {
        self.max.as_ref()
    }

    pub fn crit(&self) -> Option<&Subfeature> {
        self.crit.as_ref()
    }

    pub fn alarm(&self) -> Option<&Subfeature> {
        self.alarm.as_ref()
    }
}

#[derive(Debug)]
pub struct Core {
    pub id: u8,
    current: Option<Subfeature>,
    max: Option<Subfeature>,
    crit: Option<Subfeature>,
    alarm: Option<Subfeature>,
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

    pub fn current(&self) -> Option<&Subfeature> {
        self.current.as_ref()
    }

    pub fn max(&self) -> Option<&Subfeature> {
        self.max.as_ref()
    }

    pub fn crit(&self) -> Option<&Subfeature> {
        self.crit.as_ref()
    }

    pub fn alarm(&self) -> Option<&Subfeature> {
        self.alarm.as_ref()
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
    
    let cores = thermal.cpu.cores();
    
    for core in cores {
        println!("Core #{} - {}", core.unwrap().id, core.unwrap().current().unwrap().get_value().unwrap());
    }

    println!("Package - {}", thermal.cpu.package().unwrap().current().unwrap().get_value().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_type() {
        let thermal = Thermal::new();

        assert_eq!(thermal.cpu.package().is_none(), false);
    }

    #[test]
    fn package_thermals() {
        let thermal = Thermal::new();

        if thermal.cpu.package().is_some() {
            assert_eq!(thermal.cpu.package().unwrap().current().is_some(), true);
            assert_eq!(thermal.cpu.package().unwrap().crit().is_some(), true);
            assert_eq!(thermal.cpu.package().unwrap().max().is_some(), true);
            assert_eq!(thermal.cpu.package().unwrap().alarm().is_some(), true);
        }
    }

    #[test]
    fn core_type() {
        let thermal = Thermal::new();

        let cores = thermal.cpu.cores();
        
        for core in cores {
            assert_eq!(core.is_none(), false);
        }
    }

    #[test]
    fn core_thermals() {
        let thermal = Thermal::new();

        let cores = thermal.cpu.cores();
        
        for core in cores {
            if core.is_some() {
                assert_eq!(core.unwrap().current().is_some(), true);
                assert_eq!(core.unwrap().crit().is_some(), true);
                assert_eq!(core.unwrap().max().is_some(), true);
                assert_eq!(core.unwrap().alarm().is_some(), true);
            }
        }
    }
}
