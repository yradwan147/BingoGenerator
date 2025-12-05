// Bingo Generator - Frontend Logic

const { invoke } = window.__TAURI__.core;

// DOM Elements
const numCardsInput = document.getElementById('num-cards');
const minNumInput = document.getElementById('min-num');
const maxNumInput = document.getElementById('max-num');
const generateBtn = document.getElementById('generate-btn');
const printBtn = document.getElementById('print-btn');
const cardsContainer = document.getElementById('cards-container');
const statsPanel = document.getElementById('stats-panel');
const statsContent = document.getElementById('stats-content');
const loadingOverlay = document.getElementById('loading-overlay');
const toast = document.getElementById('toast');

// State
let currentCards = [];

// Toast notification
function showToast(message, type = 'success') {
  toast.textContent = message;
  toast.className = `toast ${type} show`;
  
  setTimeout(() => {
    toast.classList.remove('show');
  }, 3000);
}

// Show/hide loading
function setLoading(isLoading) {
  loadingOverlay.style.display = isLoading ? 'flex' : 'none';
}

// Render a single bingo card
function renderCard(card, index) {
  const cardEl = document.createElement('div');
  cardEl.className = 'bingo-card';
  cardEl.style.animationDelay = `${index * 0.05}s`;
  
  const header = document.createElement('div');
  header.className = 'card-header';
  header.innerHTML = `
    <span class="card-title">Card #${card.id}</span>
    <span class="card-badge">4×4</span>
  `;
  
  const grid = document.createElement('div');
  grid.className = 'bingo-grid';
  
  // Flatten the 2D array and create cells
  for (let row = 0; row < 4; row++) {
    for (let col = 0; col < 4; col++) {
      const cell = document.createElement('div');
      cell.className = 'bingo-cell';
      cell.textContent = card.cells[row][col];
      grid.appendChild(cell);
    }
  }
  
  cardEl.appendChild(header);
  cardEl.appendChild(grid);
  
  return cardEl;
}

// Render all cards
function renderCards(cards) {
  cardsContainer.innerHTML = '';
  
  if (cards.length === 0) {
    cardsContainer.innerHTML = `
      <div class="empty-state">
        <div class="empty-icon">
          <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <rect x="3" y="3" width="18" height="18" rx="2" stroke="currentColor" stroke-width="2"/>
            <path d="M3 9H21M9 21V9" stroke="currentColor" stroke-width="2"/>
          </svg>
        </div>
        <h3>No Cards Generated Yet</h3>
        <p>Configure your settings and click "Generate Cards" to create unique bingo cards</p>
      </div>
    `;
    return;
  }
  
  const grid = document.createElement('div');
  grid.className = 'cards-grid';
  
  cards.forEach((card, index) => {
    grid.appendChild(renderCard(card, index));
  });
  
  cardsContainer.appendChild(grid);
}

// Render distribution stats
function renderStats(distribution) {
  if (!distribution || distribution.length === 0) {
    statsPanel.style.display = 'none';
    return;
  }
  
  statsPanel.style.display = 'block';
  
  // Sort by number
  const sorted = [...distribution].sort((a, b) => a[0] - b[0]);
  
  // Calculate min/max counts
  const counts = sorted.map(d => d[1]);
  const minCount = Math.min(...counts);
  const maxCount = Math.max(...counts);
  
  statsContent.innerHTML = `
    <div class="stats-summary" style="margin-bottom: 12px; padding: 8px; background: var(--color-bg-card); border-radius: 6px;">
      <div style="display: flex; justify-content: space-between; font-size: 0.75rem; color: var(--color-text-secondary);">
        <span>Min: <strong style="color: var(--color-accent-primary)">${minCount}×</strong></span>
        <span>Max: <strong style="color: var(--color-accent-primary)">${maxCount}×</strong></span>
        <span>Range: <strong style="color: var(--color-accent-primary)">${maxCount - minCount}</strong></span>
      </div>
    </div>
    <div class="stats-content">
      ${sorted.map(([num, count]) => `
        <div class="stat-item" title="Number ${num} appears ${count} times">
          <div class="stat-number">${num}</div>
          <div class="stat-count">${count}×</div>
        </div>
      `).join('')}
    </div>
  `;
}

// Generate cards
async function generateCards() {
  const numCards = parseInt(numCardsInput.value, 10);
  const minNum = parseInt(minNumInput.value, 10);
  const maxNum = parseInt(maxNumInput.value, 10);
  
  // Validation
  if (isNaN(numCards) || numCards < 1 || numCards > 100) {
    showToast('Number of cards must be between 1 and 100', 'error');
    return;
  }
  
  if (isNaN(minNum) || isNaN(maxNum)) {
    showToast('Please enter valid numbers for the range', 'error');
    return;
  }
  
  if (maxNum <= minNum) {
    showToast('Maximum number must be greater than minimum', 'error');
    return;
  }
  
  if ((maxNum - minNum + 1) < 16) {
    showToast('Number range must be at least 16 to fill a 4×4 card', 'error');
    return;
  }
  
  setLoading(true);
  
  try {
    // Small delay to show loading animation
    await new Promise(resolve => setTimeout(resolve, 100));
    
    const result = await invoke('generate_cards', {
      numCards,
      minNum,
      maxNum
    });
    
    if (result.success) {
      currentCards = result.cards;
      renderCards(currentCards);
      renderStats(result.number_distribution);
      printBtn.style.display = 'flex';
      showToast(result.message);
    } else {
      showToast(result.message, 'error');
      currentCards = [];
      renderCards([]);
      statsPanel.style.display = 'none';
      printBtn.style.display = 'none';
    }
  } catch (error) {
    console.error('Error generating cards:', error);
    showToast('Failed to generate cards. Please try again.', 'error');
  } finally {
    setLoading(false);
  }
}

// Print cards
function printCards() {
  window.print();
}

// Event Listeners
generateBtn.addEventListener('click', generateCards);
printBtn.addEventListener('click', printCards);

// Enter key support for inputs
[numCardsInput, minNumInput, maxNumInput].forEach(input => {
  input.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
      generateCards();
    }
  });
});

// Input validation
numCardsInput.addEventListener('input', (e) => {
  let value = parseInt(e.target.value, 10);
  if (value > 100) e.target.value = 100;
  if (value < 1) e.target.value = 1;
});

// Auto-generate on load with default values
document.addEventListener('DOMContentLoaded', () => {
  // Optional: Auto-generate on load
  // generateCards();
});
