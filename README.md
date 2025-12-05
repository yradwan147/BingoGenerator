# Bingo Generator

A desktop application built with Tauri + Rust that generates unique, fair bingo cards with guaranteed balanced distribution and no duplicate winning combinations.

## Features

- **30 Bingo Cards** (configurable from 1-100)
- **4×4 Grid** per card (16 cells each)
- **Number Range 1-30** (configurable, minimum range of 16)
- **No Duplicate Wins**: Guarantees that no two players can get bingo at the same time with the same 4 numbers
- **Balanced Distribution**: Numbers are distributed evenly across all cards
- **Print Support**: Print all cards for physical use
- **Beautiful UI**: Modern dark theme with smooth animations

## How It Works

### The Algorithm

1. **Weighted Selection**: Numbers that have been used less frequently are weighted higher during selection, ensuring even distribution across all cards.

2. **Winning Line Tracking**: The algorithm tracks all possible winning lines (4 rows + 4 columns + 2 diagonals = 10 per card). Before accepting a card, it verifies none of its winning lines conflict with previously generated cards.

3. **Optimization**: Multiple generation attempts are made, keeping the result with the lowest variance in number distribution.

### Constraints

- Each card contains 16 unique numbers from the specified range
- With 30 cards × 16 cells = 480 total placements
- With 30 numbers, each appears approximately 16 times (480 ÷ 30)
- All 300 winning lines (30 cards × 10 lines) are unique

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) (latest stable)
- Platform-specific dependencies for Tauri:
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Microsoft Visual Studio C++ Build Tools
  - **Linux**: `webkit2gtk` and related packages

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd BingoGenerator

# Install dependencies
npm install
```

### Running the App

```bash
# Development mode (with hot-reload)
npm run tauri dev

# Build for production
npm run tauri build
```

## Usage

1. **Configure Settings**:
   - Set the number of cards (1-100)
   - Set the number range (minimum span of 16)

2. **Generate Cards**: Click the "Generate Cards" button

3. **View Results**: 
   - Cards are displayed in a responsive grid
   - Distribution stats show how many times each number appears

4. **Print**: Click "Print All" to print the cards

## Tech Stack

- **Frontend**: Vanilla HTML, CSS, JavaScript
- **Backend**: Rust
- **Framework**: [Tauri](https://tauri.app/) v2

## Project Structure

```
BingoGenerator/
├── src/                    # Frontend files
│   ├── index.html
│   ├── styles.css
│   └── main.js
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── lib.rs          # Bingo generation algorithm
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
└── package.json
```

## License

MIT
