pub fn to_human_readable(number: usize) -> String {
    const SI_PETA: usize = 1000 * 1000 * 1000 * 1000 * 1000;
    const SI_TERA: usize = 1000 * 1000 * 1000 * 1000;
    const SI_GIGA: usize = 1000 * 1000 * 1000;
    const SI_MEGA: usize = 1000 * 1000;
    const SI_KILO: usize = 1000;

    if number >= SI_PETA {
        format!("{} P", number / SI_PETA)
    } else if number >= SI_TERA {
        format!("{} T", number / SI_TERA)
    } else if number >= SI_GIGA {
        format!("{} G", number / SI_GIGA)
    } else if number >= SI_MEGA {
        format!("{} M", number / SI_MEGA)
    } else if number >= SI_KILO {
        format!("{} K", number / SI_KILO)
    } else {
        format!("{}", number)
    }
}
