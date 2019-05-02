use sensors::Sensors;

pub fn init() {
    let sensors = Sensors::new();

    for chip in sensors {
        println!(
            "{} (on {})",
            chip.get_name().unwrap(),
            chip.bus().get_adapter_name().unwrap()
        );
        for feature in chip {
            println!("  - {}", feature.get_label().unwrap());
            for subfeature in feature {
                println!(
                    "   - {} = {}",
                    subfeature.name(),
                    subfeature.get_value().unwrap()
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
}
