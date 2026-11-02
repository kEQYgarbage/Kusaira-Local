// Kusaira Local Anime — Desktop Frontend Controller

let currentAnime = null;
let currentEpisode = 1;
let currentReleases = [];
let selectedReleaseTopicId = null;
let currentLibFilter = 'all';

const animeDatabase = new Map();
const userLibrary = new Map(); // animeId -> LibraryEntry

function callRust(cmd, payload = {}) {
  if (window.ipc) {
    window.ipc.postMessage(JSON.stringify({ cmd, ...payload }));
  } else {
    console.log("Mock IPC:", cmd, payload);
  }
}

function navTo(view) {
  document.querySelectorAll('.view-section').forEach(s => s.classList.remove('active'));
  document.querySelectorAll('.nav-btn').forEach(b => b.classList.remove('active'));

  const catBtn = document.getElementById('nav-btn-cat');
  const libBtn = document.getElementById('nav-btn-lib');

  if (view === 'catalog') {
    document.getElementById('view-catalog').classList.add('active');
    if (catBtn) catBtn.classList.add('active');
  } else if (view === 'library') {
    document.getElementById('view-library').classList.add('active');
    if (libBtn) libBtn.classList.add('active');
    renderLibraryGrid();
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

  // 1. Instant local search across loaded catalog
  const localMatches = Array.from(animeDatabase.values()).filter(a => {
    return (a.title && a.title.toLowerCase().includes(query)) ||
           (a.name && a.name.toLowerCase().includes(query));
  });
  renderSearchResults(localMatches);

  // 2. Query live Shikimori API via Rust backend for complete coverage
  callRust("search", { query: q });
}

function openDetails(animeId) {
  const idNum = Number(animeId);
  if (animeDatabase.has(idNum)) {
    setDetails(animeDatabase.get(idNum));
  }
  callRust("get_details", { id: idNum });
  callRust("get_related", { id: idNum });
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

  const ptsPill = document.getElementById('detail-active-pts');
  if (ptsPill) ptsPill.innerText = `${anime.pts || 96} PTS`;

  const relEl = document.getElementById('detail-release');
  if (relEl) relEl.innerText = anime.release || '1080p BDRip HEVC FLAC';

  const descEl = document.getElementById('detail-desc');
  if (descEl) descEl.innerText = anime.description || '';

  updateDetailLibButton(anime.id);

  const total = anime.episodes || 12;
  const grid = document.getElementById('episodes-grid');
  if (grid) {
    grid.innerHTML = Array.from({length: total}, (_, i) => i + 1).map(n => `
      <button class="ep-btn ${n === currentEpisode ? 'active' : ''}" onclick="playEpisode(${n})">${n}</button>
    `).join('');
  }

  // Request torrent releases for this title
  callRust("get_releases", { title: anime.title || anime.name });
}

/* Related Anime (Franchise) */
function renderRelatedAnime(items) {
  const container = document.getElementById('detail-related-list');
  const section = document.getElementById('detail-related-section');
  if (!container || !section) return;

  if (!items || items.length === 0) {
    section.style.display = 'none';
    return;
  }

  section.style.display = 'block';
  container.innerHTML = items.map(rel => {
    const a = rel.anime;
    if (!a) return '';
    const poster = a.image && a.image.original 
      ? `https://shikimori.io${a.image.original}`
      : 'https://placehold.co/100x140/1f232b/fff?text=Anime';
    const title = a.russian || a.name;
    const tag = rel.relation_russian || rel.relation || 'Франшиза';

    return `
      <div class="related-card" onclick="openDetails(${a.id})">
        <img class="related-poster" src="${poster}" alt="${title}">
        <div class="related-info">
          <div class="related-tag">${tag}</div>
          <div class="related-title">${title}</div>
        </div>
      </div>
    `;
  }).join('');
}

/* Library Management */
function toggleLibDropdown() {
  const menu = document.getElementById('lib-dropdown-menu');
  if (menu) {
    menu.style.display = menu.style.display === 'block' ? 'none' : 'block';
  }
}

function updateDetailLibButton(animeId) {
  const entry = userLibrary.get(Number(animeId));
  const icon = document.getElementById('detail-lib-icon');
  const text = document.getElementById('detail-lib-text');
  if (!entry) {
    if (icon) icon.innerText = '➕';
    if (text) text.innerText = 'В библиотеку';
    return;
  }

  const map = {
    watching: { icon: '👁️', text: 'Смотрю' },
    planned: { icon: '⏳', text: 'В планах' },
    completed: { icon: '✅', text: 'Просмотрено' },
    favorite: { icon: '⭐', text: 'В избранном' },
  };
  const item = map[entry.status] || { icon: '📌', text: 'В библиотеке' };
  if (icon) icon.innerText = item.icon;
  if (text) text.innerText = item.text;
}

function setAnimeStatus(status) {
  if (!currentAnime) return;
  toggleLibDropdown();

  const entry = {
    anime_id: Number(currentAnime.id),
    title: currentAnime.title,
    name: currentAnime.name || '',
    poster: currentAnime.poster,
    status: status,
    score: currentAnime.score || '8.5',
    pts: currentAnime.pts || 95,
    episodes: currentAnime.episodes || 12,
    watched_episodes: 0,
    updated_at: Math.floor(Date.now() / 1000),
  };

  userLibrary.set(entry.anime_id, entry);
  updateDetailLibButton(entry.anime_id);
  callRust("library_add", { entry });
}

function removeAnimeFromLibrary() {
  if (!currentAnime) return;
  toggleLibDropdown();
  userLibrary.delete(Number(currentAnime.id));
  updateDetailLibButton(currentAnime.id);
  callRust("library_remove", { id: currentAnime.id });
}

function filterLibrary(status) {
  currentLibFilter = status;
  document.querySelectorAll('.lib-tab').forEach(t => t.classList.remove('active'));
  const btn = document.getElementById(`tab-lib-${status}`);
  if (btn) btn.classList.add('active');
  renderLibraryGrid();
}

function renderLibraryGrid() {
  const container = document.getElementById('shelf-library');
  const empty = document.getElementById('library-empty');
  if (!container) return;

  const allEntries = Array.from(userLibrary.values());
  const filtered = currentLibFilter === 'all'
    ? allEntries
    : allEntries.filter(e => e.status === currentLibFilter);

  if (filtered.length === 0) {
    container.innerHTML = '';
    if (empty) empty.style.display = 'block';
    return;
  }

  if (empty) empty.style.display = 'none';
  container.innerHTML = filtered.map(a => `
    <div class="card" onclick="openDetails(${a.anime_id})">
      <div class="card-poster">
        <img src="${a.poster}" onerror="this.src='https://placehold.co/300x420/1f232b/fff?text=Anime'" alt="${a.title}">
        <span class="poster-badge">${statusLabel(a.status)}</span>
      </div>
      <div class="card-title">${a.title}</div>
      <div class="card-meta">★ ${a.score} • ${a.episodes} эп.</div>
    </div>
  `).join('');
}

function statusLabel(s) {
  switch(s) {
    case 'watching': return '👁️ Смотрю';
    case 'planned': return '⏳ В планах';
    case 'completed': return '✅ Просмотрено';
    case 'favorite': return '⭐ Избранное';
    default: return s;
  }
}

/* Torrent Release Picker */
function openReleasesModal() {
  const modal = document.getElementById('releases-modal');
  if (modal) modal.style.display = 'flex';
  renderReleasesList(currentReleases);
}

function closeReleasesModal() {
  const modal = document.getElementById('releases-modal');
  if (modal) modal.style.display = 'none';
}

function setReleases(releases) {
  currentReleases = releases || [];
  if (releases && releases.length > 0 && !selectedReleaseTopicId) {
    selectedReleaseTopicId = releases[0].topic_id;
  }
}

function renderReleasesList(releases) {
  const list = document.getElementById('releases-modal-list');
  if (!list) return;

  if (!releases || releases.length === 0) {
    list.innerHTML = '<div style="color: var(--text-muted); padding: 12px;">Поиск активных раздач...</div>';
    return;
  }

  list.innerHTML = releases.map(r => {
    const isActive = selectedReleaseTopicId === r.topic_id;
    return `
      <div class="release-card ${isActive ? 'active' : ''}">
        <div>
          <div class="release-title">${r.title}</div>
          <div class="release-meta">
            <span style="color: var(--accent-primary); font-weight: 700;">${r.pts} PTS</span> • 
            <span>${r.size_human || r.size_formatted || '12 GB'}</span> • 
            <span>${r.seeders} сидов</span>
          </div>
        </div>
        <button class="btn-choose-release" onclick="chooseRelease('${r.topic_id}', ${r.pts}, '${escapeQuotes(r.title)}')">
          ${isActive ? '✓ Активен' : 'Выбрать'}
        </button>
      </div>
    `;
  }).join('');
}

function chooseRelease(topicId, pts, title) {
  selectedReleaseTopicId = topicId;
  const relEl = document.getElementById('detail-release');
  if (relEl) relEl.innerText = title;

  const ptsEl = document.getElementById('detail-pts');
  if (ptsEl) ptsEl.innerText = `${pts} PTS`;

  const ptsPill = document.getElementById('detail-active-pts');
  if (ptsPill) ptsPill.innerText = `${pts} PTS`;

  closeReleasesModal();
  callRust("select_release", { topic_id: topicId });
}

function escapeQuotes(str) {
  return (str || '').replace(/'/g, "\\'");
}

/* Settings Modal */
function openSettingsModal() {
  const modal = document.getElementById('settings-modal');
  if (modal) modal.style.display = 'flex';
}

function closeSettingsModal() {
  const modal = document.getElementById('settings-modal');
  if (modal) modal.style.display = 'none';
}

function saveSettings() {
  const voiceover = document.getElementById('set-voiceover').value;
  const quality = document.getElementById('set-quality').value;
  const hwaccel = document.getElementById('set-hwaccel').value;

  callRust("save_settings", { settings: { voiceover, quality, hwaccel } });
  closeSettingsModal();
}

function closeModalOnBg(e, modalId) {
  if (e.target.id === modalId) {
    document.getElementById(modalId).style.display = 'none';
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

// Initial requests
window.addEventListener('DOMContentLoaded', () => {
  callRust("get_catalog");
  callRust("library_list");
});
