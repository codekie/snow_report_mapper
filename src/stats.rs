/// This module keeps and modifies statistics
use std::cmp;
use std::collections::HashMap;

/// If the terminal width can't be determined, use this width
const DEFAULT_MAX_WIDTH_HISTOGRAM: u16 = 80;

/// Keeps statistics that were collected during the conversion
#[derive(Clone)]
pub struct Stats {
    /// Distribution of keys, with occurrences. The keys consist of the original key with the mapped
    /// OpenAI category appended as suffix ` [ID]` Whereas `ID` is the numeric category-ID
    pub distribution: HashMap<String, u16>,
}

impl<'a> Stats {
    pub fn new() -> Self {
        Stats {
            distribution: HashMap::new(),
        }
    }

    /// Increase the occurrence of a key by one
    ///
    /// # Arguments
    ///
    /// - `key`: Key which occurrence has to be increased
    pub fn inc_distribution<'b>(&'b mut self, key: &'a String, category: usize) {
        let display_name: String = format!("{} [{}]", key, category);
        let group_count = self.distribution.get(&display_name).unwrap_or(&0);
        self.distribution.insert(display_name, group_count + 1);
    }

    /// Prints stats to console.
    ///
    /// This contains:
    ///
    /// - A histogram of the distribution of keys
    pub fn print_stats(&self) {
        print_key_histogram(&self.distribution);
        println!();
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

/// Determines the width of the terminal
///
/// # Returns
///
/// The width of the terminal. `DEFAULT_MAX_WIDTH_HISTOGRAM` if it can't be determined.
fn get_terminal_width() -> u16 {
    let max_width_raw = termsize::get();
    match max_width_raw.is_some() {
        true => max_width_raw.unwrap().cols,
        false => DEFAULT_MAX_WIDTH_HISTOGRAM,
    }
}

/// Prints a histogram of the distribution of keys
///
/// # Arguments
///
/// - `distribution`: distribution of keys
fn print_key_histogram(distribution: &HashMap<String, u16>) {
    let mut ordered_entries: Vec<(&String, &u16)> = distribution.iter().collect();
    ordered_entries.sort_by(|(_, amount1), (_, amount2)| amount2.cmp(amount1));
    let terminal_width = get_terminal_width();
    println!("{} distinct categories are in use", ordered_entries.len());
    // Print header
    println!(
        "\n{:=^width$}",
        " Assignment group distribution ",
        width = terminal_width as usize
    );
    // Print histogram
    let (max_key_len, max_amount) = get_max_values(&ordered_entries);
    for (name, amount) in &ordered_entries {
        let available_width_for_histogram = terminal_width - max_key_len - 2;
        let histogram_padding = (**amount as f64 / max_amount as f64
            * available_width_for_histogram as f64)
            .round() as usize;
        println!(
            "{: >key_width$}: {: >value_padding$}",
            name,
            amount,
            value_padding = histogram_padding,
            key_width = max_key_len as usize
        );
    }
}

/// Gets relevant max-values for the statistics output
///
/// # Arguments
///
/// - `entries`: Keys and the amount of occurrences
///
/// # Returns
///
/// Tuple with:
///
/// - `max_key_len`: Length of the longest key
/// - `max_amount`: Highest amount of occurrences
fn get_max_values(entries: &Vec<(&String, &u16)>) -> (u16, u16) {
    let mut max_key_len: u16 = 0;
    let mut max_amount: u16 = 0;
    for (name, amount) in entries {
        max_key_len = cmp::max(max_key_len, name.len() as u16);
        max_amount = cmp::max(max_amount, **amount);
    }
    (max_key_len, max_amount)
}
