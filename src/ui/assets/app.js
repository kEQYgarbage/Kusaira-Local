// Kusaira Local Anime — Desktop Frontend Controller

let currentAnime = null;
let currentEpisode = 1;
const animeDatabase = new Map();

function callRust(cmd, payload = {}) {
  if (window.ipc) {
    window.ipc.postMessage(JSON.stringify({ cmd, ...payload }));
  } else {
    console.log("Mock IPC:", cmd, payload);
  }
}

function navTo(view) {
  document.querySelectorAll('.view-section').forEach(s => s.classList.remove('active'));
  const catBtn = document.getElementById('nav-btn-cat');
  if (catBtn) catBtn.classList.toggle('active', view === 'catalog');

  if (view === 'catalog') {
    document.getElementById('view-catalog').classList.add('active');
  } else {
    document.getElementById('view-detail').classList.add('active');
  }
}

function renderShelf(containerId, items) {
  const el = document.getElementById(containerId);
  if (!el || !Array.isArray(items)) return;

  items.forEach(a => animeDatabase.set(Number(a.id), a));

  el.innerHTML = items.map(a => `
    <div class="card" onclick="openDetails(${a.id})">
      <div class="card-poster">
        <img src="${a.poster}" onerror="this.src='https://placehold.co/300x420/1f232b/fff?text=Anime'" alt="${a.title}">
        <span class="poster-badge">1080p • <span class="pts-badge">${a.pts || 95} PTS</span></span>
      </div>
      <div class="card-title">${a.title}</div>
      <div class="card-meta">★ ${a.score || '8.5'} • ${a.episodes || 12} эп.</div>
    </div>
  `).join('');
}

function renderSearchResults(items) {
  const wrap = document.getElementById('shelf-search-wrap');
  const shelves = document.getElementById('catalog-shelves');
  if (!wrap) return;

  if (!items || items.length === 0) {
    wrap.style.display = 'none';
    if (shelves) shelves.style.display = 'block';
    return;
  }

  wrap.style.display = 'block';
  if (shelves) shelves.style.display = 'none';
  renderShelf('shelf-search', items);
}

function searchAnime(q) {
  const query = q.trim().toLowerCase();
  const wrap = document.getElementById('shelf-search-wrap');
  const shelves = document.getElementById('catalog-shelves');

  if (!query) {
    if (wrap) wrap.style.display = 'none';
    if (shelves) shelves.style.display = 'block';
    return;
  }

  // Instant client-side search across loaded catalog
  const matches = Array.from(animeDatabase.values()).filter(a => {
    return (a.title && a.title.toLowerCase().includes(query)) ||
           (a.name && a.name.toLowerCase().includes(query));
  });

  renderSearchResults(matches);
  callRust("search", { query: q });
}

function openDetails(animeId) {
  const idNum = Number(animeId);
  if (animeDatabase.has(idNum)) {
    setDetails(animeDatabase.get(idNum));
  }
  callRust("get_details", { id: idNum });
  navTo('detail');
}

function setDetails(anime) {
  if (!anime) return;
  currentAnime = anime;
  animeDatabase.set(Number(anime.id), anime);

  const titleEl = document.getElementById('detail-title');
  if (titleEl) titleEl.innerText = anime.title || '';

  const subEl = document.getElementById('detail-subtitle');
  if (subEl) subEl.innerText = anime.name || '';

  const imgEl = document.getElementById('detail-poster-img');
  if (imgEl) imgEl.src = anime.poster || '';

  const scoreEl = document.getElementById('detail-score');
  if (scoreEl) scoreEl.innerText = `★ ${anime.score || '8.8'}`;

  const ptsEl = document.getElementById('detail-pts');
  if (ptsEl) ptsEl.innerText = `${anime.pts || 96} PTS`;

  const relEl = document.getElementById('detail-release');
  if (relEl) relEl.innerText = anime.release || '1080p BDRip HEVC FLAC';

  const descEl = document.getElementById('detail-desc');
  if (descEl) descEl.innerText = anime.description || '';

  const total = anime.episodes || 12;
  const grid = document.getElementById('episodes-grid');
  if (grid) {
    grid.innerHTML = Array.from({length: total}, (_, i) => i + 1).map(n => `
      <button class="ep-btn ${n === currentEpisode ? 'active' : ''}" onclick="playEpisode(${n})">${n}</button>
    `).join('');
  }
}

function playEpisode(ep) {
  currentEpisode = ep;
  document.querySelectorAll('.ep-btn').forEach((b, i) => {
    b.classList.toggle('active', i + 1 === ep);
  });
  callRust("play_episode", {
    anime_id: currentAnime ? currentAnime.id : 0,
    episode: ep,
  });
}

function togglePlayPause() {
  callRust("player_action", { action: "toggle_pause" });
}

// Initial catalog request to Rust backend
window.addEventListener('DOMContentLoaded', () => {
  callRust("get_catalog");
});
