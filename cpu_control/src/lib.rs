use cpu_thermal;

pub fn init() {
    cpu_thermal::init();
}

#[cfg(test)]
mod tests {
}
