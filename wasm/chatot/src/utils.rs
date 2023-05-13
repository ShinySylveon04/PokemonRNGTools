use std::fmt::Display;

pub fn format_ivs(ivs: &[impl Display]) -> String {
    if ivs.len() != 6 {
        return String::new();
    }

    format!(
        "{} / {} / {} / {} / {} / {}",
        ivs[0], ivs[1], ivs[2], ivs[3], ivs[4], ivs[5]
    )
}
