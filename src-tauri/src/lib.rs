use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingoCard {
    pub id: usize,
    pub cells: [[u32; 4]; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResult {
    pub cards: Vec<BingoCard>,
    pub number_distribution: Vec<(u32, usize)>,
    pub success: bool,
    pub message: String,
}

/// Extract all winning lines from a bingo card
fn get_winning_lines(card: &[[u32; 4]; 4]) -> Vec<Vec<u32>> {
    let mut lines = Vec::new();

    // Rows
    for row in card.iter() {
        let mut line: Vec<u32> = row.to_vec();
        line.sort();
        lines.push(line);
    }

    // Columns
    for col in 0..4 {
        let mut line: Vec<u32> = (0..4).map(|row| card[row][col]).collect();
        line.sort();
        lines.push(line);
    }

    // Main diagonal (top-left to bottom-right)
    let mut diag1: Vec<u32> = (0..4).map(|i| card[i][i]).collect();
    diag1.sort();
    lines.push(diag1);

    // Anti-diagonal (top-right to bottom-left)
    let mut diag2: Vec<u32> = (0..4).map(|i| card[i][3 - i]).collect();
    diag2.sort();
    lines.push(diag2);

    lines
}

/// Convert a winning line to a unique string key for HashSet
fn line_to_key(line: &[u32]) -> String {
    line.iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

/// Generate bingo cards with the specified constraints
fn generate_bingo_cards(
    num_cards: usize,
    min_num: u32,
    max_num: u32,
    max_attempts: usize,
) -> GenerationResult {
    let numbers: Vec<u32> = (min_num..=max_num).collect();
    let num_range = numbers.len();

    // Each card has 16 cells, we have num_cards cards
    // Target: each number should appear approximately (16 * num_cards) / num_range times
    let total_cells = 16 * num_cards;
    let target_per_number = total_cells / num_range;

    let mut rng = rand::thread_rng();
    let mut best_result: Option<(Vec<BingoCard>, Vec<usize>)> = None;
    let mut best_variance = f64::MAX;

    for _attempt in 0..max_attempts {
        let mut cards: Vec<BingoCard> = Vec::new();
        let mut used_lines: HashSet<String> = HashSet::new();
        let mut number_counts: Vec<usize> = vec![0; num_range];
        let mut success = true;

        for card_id in 0..num_cards {
            let mut card_found = false;
            let mut card_attempts = 0;
            const MAX_CARD_ATTEMPTS: usize = 1000;

            while !card_found && card_attempts < MAX_CARD_ATTEMPTS {
                card_attempts += 1;

                // Select 16 numbers for this card using weighted random selection
                let mut selected: Vec<u32> = Vec::new();
                let mut temp_counts = number_counts.clone();

                while selected.len() < 16 {
                    // Recalculate weights based on current temp_counts
                    let current_weights: Vec<f64> = temp_counts
                        .iter()
                        .enumerate()
                        .map(|(i, &c)| {
                            if selected.contains(&numbers[i]) {
                                0.0 // Can't select same number twice on same card
                            } else {
                                let diff = (target_per_number as f64) - (c as f64);
                                (diff + 10.0).max(0.1)
                            }
                        })
                        .collect();

                    let total_weight: f64 = current_weights.iter().sum();
                    if total_weight <= 0.0 {
                        break;
                    }

                    let mut random_val = rng.gen::<f64>() * total_weight;
                    for (i, &w) in current_weights.iter().enumerate() {
                        random_val -= w;
                        if random_val <= 0.0 {
                            selected.push(numbers[i]);
                            temp_counts[i] += 1;
                            break;
                        }
                    }
                }

                if selected.len() != 16 {
                    continue;
                }

                // Shuffle and arrange into 4x4 grid
                selected.shuffle(&mut rng);
                let card: [[u32; 4]; 4] = [
                    [selected[0], selected[1], selected[2], selected[3]],
                    [selected[4], selected[5], selected[6], selected[7]],
                    [selected[8], selected[9], selected[10], selected[11]],
                    [selected[12], selected[13], selected[14], selected[15]],
                ];

                // Check if any winning lines conflict with existing ones
                let lines = get_winning_lines(&card);
                let mut has_conflict = false;

                for line in &lines {
                    let key = line_to_key(line);
                    if used_lines.contains(&key) {
                        has_conflict = true;
                        break;
                    }
                }

                if !has_conflict {
                    // Card is valid, add it
                    for line in &lines {
                        used_lines.insert(line_to_key(line));
                    }

                    // Update number counts
                    for &num in &selected {
                        let idx = (num - min_num) as usize;
                        number_counts[idx] += 1;
                    }

                    cards.push(BingoCard {
                        id: card_id + 1,
                        cells: card,
                    });
                    card_found = true;
                }
            }

            if !card_found {
                success = false;
                break;
            }
        }

        if success {
            // Calculate variance of distribution
            let mean = number_counts.iter().sum::<usize>() as f64 / num_range as f64;
            let variance: f64 = number_counts
                .iter()
                .map(|&c| {
                    let diff = c as f64 - mean;
                    diff * diff
                })
                .sum::<f64>()
                / num_range as f64;

            if variance < best_variance {
                best_variance = variance;
                best_result = Some((cards, number_counts));
            }

            // If we have a good enough distribution, stop early
            if variance < 2.0 {
                break;
            }
        }
    }

    match best_result {
        Some((cards, counts)) => {
            let distribution: Vec<(u32, usize)> = numbers
                .iter()
                .zip(counts.iter())
                .map(|(&n, &c)| (n, c))
                .collect();

            GenerationResult {
                cards,
                number_distribution: distribution,
                success: true,
                message: format!(
                    "Successfully generated {} bingo cards with balanced distribution!",
                    num_cards
                ),
            }
        }
        None => GenerationResult {
            cards: Vec::new(),
            number_distribution: Vec::new(),
            success: false,
            message: "Failed to generate valid bingo cards. Try adjusting parameters.".to_string(),
        },
    }
}

#[tauri::command]
fn generate_cards(num_cards: usize, min_num: u32, max_num: u32) -> GenerationResult {
    // Validate inputs
    if max_num < min_num {
        return GenerationResult {
            cards: Vec::new(),
            number_distribution: Vec::new(),
            success: false,
            message: "Maximum number must be greater than or equal to minimum number.".to_string(),
        };
    }

    let range = (max_num - min_num + 1) as usize;
    if range < 16 {
        return GenerationResult {
            cards: Vec::new(),
            number_distribution: Vec::new(),
            success: false,
            message: "Number range must be at least 16 to fill a 4x4 card.".to_string(),
        };
    }

    generate_bingo_cards(num_cards, min_num, max_num, 50)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![generate_cards])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
