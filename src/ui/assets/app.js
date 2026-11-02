// Kusaira Local Anime — Desktop Frontend Controller

let currentAnime = null, currentEpisode = 1, currentReleases = [], selectedReleaseTopicId = null;
let currentLibFilter = 'all', currentOpeningActive = false;
const animeDatabase = new Map(), userLibrary = new Map();

let appConfig = {
  theme: 'cyan', grid_density: 'standard', voiceover_priority: 'Studio Band',
  quality_preference: '1080p', hw_accel: 'vaapi', opening_skip_mode: 'tab',
  skip_endings: false, seek_step_sec: 10, autoplay_next: true, resume_playback: true,
  gc_max_disk_gb: 20, gc_policy: 'keep_3', gc_on_exit: true, discord_rpc_enabled: true,
};

function callRust(cmd, payload = {}) {
  if (window.ipc) window.ipc.postMessage(JSON.stringify({ cmd, ...payload }));
  else console.log("Mock IPC:", cmd, payload);
}

function navTo(view) {
  document.querySelectorAll('.view-section').forEach(s => s.classList.remove('active'));
  document.querySelectorAll('.nav-btn').forEach(b => b.classList.remove('active'));
  const catBtn = document.getElementById('nav-btn-cat'), libBtn = document.getElementById('nav-btn-lib');

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
      <div class="card-meta"><svg class="icon star-icon" viewBox="0 0 24 24"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon></svg> ${a.score || '8.5'} • ${a.episodes || 12} эп.</div>
    </div>
  `).join('');
}

function renderSearchResults(items) {
  const wrap = document.getElementById('shelf-search-wrap'), shelves = document.getElementById('catalog-shelves');
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

const shikiCache = new Map();
async function cachedFetchJson(url) {
  if (shikiCache.has(url)) return shikiCache.get(url);
  try {
    const res = await fetch(url);
    if (!res.ok) return null;
    const json = await res.json();
    shikiCache.set(url, json);
    return json;
  } catch (_) { return null; }
}

function mapShikiToCard(s) {
  const p = s.image?.original ? `https://shikimori.io${s.image.original}` : (s.image?.preview ? `https://shikimori.io${s.image.preview}` : 'https://placehold.co/300x420/1f232b/fff?text=Anime');
  return {
    id: s.id, title: s.russian || s.name, name: s.name, poster: p, score: s.score || '8.0',
    pts: 95, episodes: s.episodes || (s.kind === 'movie' ? 1 : 12), release: '1080p BDRip HEVC FLAC',
    description: `Аниме: ${s.russian || s.name}. Рейтинг: ${s.score || '—'}. Тип: ${(s.kind || 'TV').toUpperCase()}.`,
  };
}

let searchTimer = null;
function searchAnime(q) {
  const query = q.trim(), wrap = document.getElementById('shelf-search-wrap'), shelves = document.getElementById('catalog-shelves');
  if (!query) {
    if (wrap) wrap.style.display = 'none';
    if (shelves) shelves.style.display = 'block';
    return;
  }
  const local = Array.from(animeDatabase.values()).filter(a => (a.title && a.title.toLowerCase().includes(query.toLowerCase())) || (a.name && a.name.toLowerCase().includes(query.toLowerCase())));
  if (local.length > 0) renderSearchResults(local);
  clearTimeout(searchTimer);
  searchTimer = setTimeout(async () => {
    const data = await cachedFetchJson(`https://shikimori.io/api/animes?search=${encodeURIComponent(query)}&limit=50&order=popularity`);
    if (Array.isArray(data) && data.length > 0) { renderSearchResults(data.map(mapShikiToCard)); return; }
    callRust("search", { query });
  }, 240);
}

async function loadCategory(cat, btn) {
  if (btn) { document.querySelectorAll('.cat-pill').forEach(p => p.classList.remove('active')); btn.classList.add('active'); }
  const g = { genre_action: 'genre=1', genre_fantasy: 'genre=10', genre_romance: 'genre=22', genre_comedy: 'genre=4', genre_scifi: 'genre=24', movie: 'kind=movie', ongoing: 'status=ongoing', ranked: 'order=ranked' };
  const data = await cachedFetchJson(`https://shikimori.io/api/animes?${g[cat] || 'order=popularity'}&limit=24`);
  if (Array.isArray(data)) renderShelf('shelf-ongoing', data.map(mapShikiToCard));
}

async function openDetails(animeId) {
  const idNum = Number(animeId);
  selectedReleaseTopicId = null;
  if (animeDatabase.has(idNum)) setDetails(animeDatabase.get(idNum));
  navTo('detail');
  callRust("get_details", { id: idNum });
  callRust("get_related", { id: idNum });
  const [resDet, resRel] = await Promise.all([
    cachedFetchJson(`https://shikimori.io/api/animes/${idNum}`),
    cachedFetchJson(`https://shikimori.io/api/animes/${idNum}/related`)
  ]);
  if (resDet) {
    const full = mapShikiToCard(resDet);
    if (resDet.description) full.description = resDet.description;
    if (resDet.videos) full.videos = resDet.videos;
    setDetails(full);
  }
  if (Array.isArray(resRel) && resRel.length > 0) renderRelatedAnime(resRel);
}

function setDetails(anime) {
  if (!anime) return;
  currentAnime = anime;
  animeDatabase.set(Number(anime.id), anime);
  const s = (id, val) => { const el = document.getElementById(id); if (el) el.innerText = val; };
  s('detail-title', anime.title || ''); s('detail-subtitle', anime.name || '');
  s('detail-score-val', anime.score || '8.8'); s('detail-pts', `${anime.pts || 96} PTS`);
  s('detail-active-pts', `${anime.pts || 96} PTS`); s('detail-release', anime.release || '1080p BDRip HEVC FLAC');
  s('detail-desc', anime.description || '');
  const img = document.getElementById('detail-poster-img'); if (img) img.src = anime.poster || '';
  updateDetailLibButton(anime.id);
  const total = anime.episodes || 12, grid = document.getElementById('episodes-grid');
  if (grid) grid.innerHTML = Array.from({length: total}, (_, i) => i + 1).map(n => `<button class="ep-btn ${n === currentEpisode ? 'active' : ''}" onclick="playEpisode(${n})">${n}</button>`).join('');
  callRust("get_releases", { title: anime.title || anime.name });
}

/* Related Anime (Franchise) */
function renderRelatedAnime(items) {
  const container = document.getElementById('detail-related-list'), section = document.getElementById('detail-related-section');
  if (!container || !section) return;
  if (!items || items.length === 0) { section.style.display = 'none'; return; }
  section.style.display = 'block';
  container.innerHTML = items.map(rel => {
    const a = rel.anime;
    if (!a) return '';
    const poster = a.image && a.image.original ? `https://shikimori.io${a.image.original}` : 'https://placehold.co/100x140/1f232b/fff?text=Anime';
    return `
      <div class="related-card" onclick="openDetails(${a.id})">
        <img class="related-poster" src="${poster}" alt="${a.russian || a.name}">
        <div class="related-info">
          <div class="related-tag">${rel.relation_russian || rel.relation || 'Франшиза'}</div>
          <div class="related-title">${a.russian || a.name}</div>
        </div>
      </div>
    `;
  }).join('');
}

/* Library Management */
function toggleLibDropdown() {
  const menu = document.getElementById('lib-dropdown-menu');
  if (menu) menu.style.display = menu.style.display === 'block' ? 'none' : 'block';
}

function updateDetailLibButton(animeId) {
  const entry = userLibrary.get(Number(animeId)), icon = document.getElementById('detail-lib-icon'), text = document.getElementById('detail-lib-text');
  if (!entry) {
    if (icon) icon.innerHTML = '<svg class="icon" viewBox="0 0 24 24"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>';
    if (text) text.innerText = 'В библиотеку';
    return;
  }
  const map = { watching: 'Смотрю', planned: 'В планах', completed: 'Просмотрено', favorite: 'В избранном' };
  if (text) text.innerText = map[entry.status] || 'В библиотеке';
  if (icon) icon.innerHTML = '<svg class="icon" viewBox="0 0 24 24"><polyline points="20 6 9 17 4 12"></polyline></svg>';
}

function setAnimeStatus(status) {
  if (!currentAnime) return;
  toggleLibDropdown();
  const entry = {
    anime_id: Number(currentAnime.id), title: currentAnime.title, name: currentAnime.name || '',
    poster: currentAnime.poster, status: status, score: currentAnime.score || '8.5', pts: currentAnime.pts || 95,
    episodes: currentAnime.episodes || 12, watched_episodes: 0, updated_at: Math.floor(Date.now() / 1000),
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
  const container = document.getElementById('shelf-library'), empty = document.getElementById('library-empty');
  if (!container) return;
  const all = Array.from(userLibrary.values()), filtered = currentLibFilter === 'all' ? all : all.filter(e => e.status === currentLibFilter);
  if (filtered.length === 0) {
    container.innerHTML = '';
    if (empty) empty.style.display = 'block';
    return;
  }
  if (empty) empty.style.display = 'none';
  const labelMap = { watching: 'Смотрю', planned: 'В планах', completed: 'Просмотрено', favorite: 'Избранное' };
  container.innerHTML = filtered.map(a => `
    <div class="card" onclick="openDetails(${a.anime_id})">
      <div class="card-poster">
        <img src="${a.poster}" onerror="this.src='https://placehold.co/300x420/1f232b/fff?text=Anime'" alt="${a.title}">
        <span class="poster-badge">${labelMap[a.status] || a.status}</span>
      </div>
      <div class="card-title">${a.title}</div>
      <div class="card-meta"><svg class="icon star-icon" viewBox="0 0 24 24"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon></svg> ${a.score} • ${a.episodes} эп.</div>
    </div>
  `).join('');
}

/* Releases Picker */
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
  if (releases && releases.length > 0) {
    if (!selectedReleaseTopicId || !releases.some(r => r.topic_id === selectedReleaseTopicId)) {
      selectedReleaseTopicId = releases[0].topic_id;
      const relEl = document.getElementById('detail-release'), ptsPill = document.getElementById('detail-active-pts');
      if (relEl) relEl.innerText = releases[0].title;
      if (ptsPill) ptsPill.innerText = `${releases[0].pts} PTS`;
    }
  }
}

function renderReleasesList(releases) {
  const list = document.getElementById('releases-modal-list');
  if (!list) return;
  if (!releases || releases.length === 0) {
    list.innerHTML = '<div style="color: var(--text-muted); padding: 12px;">Поиск активных раздач...</div>';
    return;
  }
  list.innerHTML = releases.map(r => `
    <div class="release-card ${selectedReleaseTopicId === r.topic_id ? 'active' : ''}">
      <div>
        <div class="release-title">${r.title}</div>
        <div class="release-meta"><span style="color: var(--accent-primary); font-weight: 700;">${r.pts} PTS</span> • <span>${r.size_human || '12 GB'}</span> • <span>${r.seeders} сидов</span></div>
      </div>
      <button class="btn-choose-release" onclick="chooseRelease('${r.topic_id}', ${r.pts}, '${(r.title || '').replace(/'/g, "\\'")}')">
        ${selectedReleaseTopicId === r.topic_id ? '<svg class="icon" viewBox="0 0 24 24"><polyline points="20 6 9 17 4 12"></polyline></svg> Активен' : 'Выбрать'}
      </button>
    </div>
  `).join('');
}

function chooseRelease(topicId, pts, title) {
  selectedReleaseTopicId = topicId;
  const relEl = document.getElementById('detail-release'), ptsPill = document.getElementById('detail-active-pts');
  if (relEl) relEl.innerText = title;
  if (ptsPill) ptsPill.innerText = `${pts} PTS`;
  closeReleasesModal();
  callRust("select_release", { topic_id: topicId });
}

/* Openings Auto-skip */
function checkOpeningAlert() {
  const mode = appConfig.opening_skip_mode || 'tab';
  if (mode === 'off') { hideOpeningBanner(); return; }
  if (mode === 'auto') { triggerOpeningSkip(); return; }
  currentOpeningActive = true;
  const banner = document.getElementById('opening-banner'), kbd = document.getElementById('opening-kbd-hint');
  if (kbd) kbd.innerText = mode === 'space' ? 'Пробел' : 'Tab';
  if (banner) banner.style.display = 'flex';
  setTimeout(() => hideOpeningBanner(), 12000);
}

function hideOpeningBanner() {
  currentOpeningActive = false;
  const banner = document.getElementById('opening-banner');
  if (banner) banner.style.display = 'none';
}

function triggerOpeningSkip() {
  hideOpeningBanner();
  const prompt = document.getElementById('player-skip-prompt');
  if (prompt) prompt.style.display = 'none';
  const v = document.getElementById('player-video');
  if (v && v.currentTime < 95) v.currentTime = 95;
  if (fallbackTimer && simulatedTime < 95) simulatedTime = 95;
  callRust("player_action", { action: "skip_forward_85s" });
}

/* Theming & Density */
function previewTheme(th) { document.body.setAttribute('data-theme', th); }
function previewDensity(d) { document.body.setAttribute('data-density', d); }

/* Settings Modal Multi-tab */
function openSettingsModal() {
  const modal = document.getElementById('settings-modal');
  if (modal) modal.style.display = 'flex';
  syncAllCustomSelects();
  callRust("get_gc_stats");
}

function closeSettingsModal() {
  const modal = document.getElementById('settings-modal');
  if (modal) modal.style.display = 'none';
}

function switchSettingsTab(tab) {
  document.querySelectorAll('.set-tab').forEach(b => b.classList.remove('active'));
  document.querySelectorAll('.settings-panel').forEach(p => p.style.display = 'none');
  const btn = document.getElementById(`tab-btn-${tab}`), panel = document.getElementById(`panel-${tab}`);
  if (btn) btn.classList.add('active');
  if (panel) panel.style.display = 'flex';
}

function applyConfig(cfg) {
  if (!cfg) return;
  appConfig = { ...appConfig, ...cfg };
  previewTheme(appConfig.theme); previewDensity(appConfig.grid_density);
  const sV = (id, val) => { const el = document.getElementById(id); if (el) el.value = val; };
  const sC = (id, val) => { const el = document.getElementById(id); if (el) el.checked = !!val; };
  sV('set-theme', appConfig.theme); sV('set-density', appConfig.grid_density); sV('set-op-skip', appConfig.opening_skip_mode);
  sV('set-seek-step', appConfig.seek_step_sec); sV('set-gc-max-gb', appConfig.gc_max_disk_gb); sV('set-gc-policy', appConfig.gc_policy);
  sV('set-voiceover', appConfig.voiceover_priority); sV('set-quality', appConfig.quality_preference); sV('set-hwaccel', appConfig.hw_accel);
  sC('set-autoplay', appConfig.autoplay_next); sC('set-resume', appConfig.resume_playback); sC('set-skip-ed', appConfig.skip_endings);
  sC('set-gc-exit', appConfig.gc_on_exit); sC('set-discord-rpc', appConfig.discord_rpc_enabled); syncAllCustomSelects();
}

function saveSettings() {
  const gV = id => document.getElementById(id)?.value, gC = id => !!document.getElementById(id)?.checked;
  Object.assign(appConfig, {
    theme: gV('set-theme'), grid_density: gV('set-density'), opening_skip_mode: gV('set-op-skip'),
    seek_step_sec: Number(gV('set-seek-step')), gc_max_disk_gb: Number(gV('set-gc-max-gb')), gc_policy: gV('set-gc-policy'),
    voiceover_priority: gV('set-voiceover'), quality_preference: gV('set-quality'), hw_accel: gV('set-hwaccel'),
    autoplay_next: gC('set-autoplay'), resume_playback: gC('set-resume'), skip_endings: gC('set-skip-ed'),
    gc_on_exit: gC('set-gc-exit'), discord_rpc_enabled: gC('set-discord-rpc'),
  });
  previewTheme(appConfig.theme); previewDensity(appConfig.grid_density);
  callRust("save_config", { config: appConfig }); closeSettingsModal();
}

/* GC Handlers */
function runGcClean(isAll) {
  callRust("run_gc", { gc_mode: isAll ? "all" : "threshold" });
}

function onGcComplete(stats) {
  const text = document.getElementById('gc-cache-size-text');
  if (text) text.innerText = `Очищено: ${formatBytes(stats.freed_bytes)}. Осталось: ${formatBytes(stats.remaining_bytes)}`;
}

function setGcCacheBytes(bytes) {
  const text = document.getElementById('gc-cache-size-text');
  if (text) text.innerText = `${formatBytes(bytes)} занято на диске`;
}

function formatBytes(bytes) {
  if (!bytes || bytes === 0) return '0 МБ';
  const mb = bytes / (1024 * 1024);
  return mb > 1024 ? `${(mb / 1024).toFixed(1)} ГБ` : `${mb.toFixed(0)} МБ`;
}

function closeModalOnBg(e, modalId) {
  if (e.target.id === modalId) document.getElementById(modalId).style.display = 'none';
}

/* Custom Dark Dropdowns */
function initCustomSelects() {
  document.querySelectorAll('.setting-select').forEach(sel => {
    if (sel.nextElementSibling?.classList.contains('custom-select-wrap')) return;
    sel.style.display = 'none';
    const wrap = document.createElement('div');
    wrap.className = 'custom-select-wrap';
    wrap.dataset.for = sel.id;
    wrap.innerHTML = `<button type="button" class="custom-select-trigger"><span class="custom-select-text"></span><svg class="icon custom-select-arrow" viewBox="0 0 24 24"><polyline points="6 9 12 15 18 9"></polyline></svg></button><div class="custom-select-menu"></div>`;
    const trigger = wrap.querySelector('.custom-select-trigger'), menu = wrap.querySelector('.custom-select-menu');
    Array.from(sel.options).forEach(opt => {
      const item = document.createElement('div');
      item.className = 'custom-select-opt';
      item.dataset.value = opt.value;
      item.innerText = opt.text;
      item.onclick = (e) => {
        e.stopPropagation();
        sel.value = opt.value;
        sel.dispatchEvent(new Event('change'));
        updateCustomSelect(wrap, sel);
        wrap.classList.remove('open');
      };
      menu.appendChild(item);
    });
    trigger.onclick = (e) => {
      e.stopPropagation();
      const open = wrap.classList.contains('open');
      document.querySelectorAll('.custom-select-wrap.open').forEach(w => w.classList.remove('open'));
      if (!open) wrap.classList.add('open');
    };
    sel.parentNode.insertBefore(wrap, sel.nextSibling);
    updateCustomSelect(wrap, sel);
  });
}

function updateCustomSelect(wrap, sel) {
  const currentVal = sel.value, selOpt = Array.from(sel.options).find(o => o.value == currentVal) || sel.options[0];
  const textEl = wrap.querySelector('.custom-select-text');
  if (textEl && selOpt) textEl.innerText = selOpt.text;
  wrap.querySelectorAll('.custom-select-opt').forEach(opt => opt.classList.toggle('selected', opt.dataset.value == currentVal));
}

function syncAllCustomSelects() {
  document.querySelectorAll('.custom-select-wrap').forEach(wrap => {
    const sel = document.getElementById(wrap.dataset.for);
    if (sel) updateCustomSelect(wrap, sel);
  });
}

document.addEventListener('click', () => document.querySelectorAll('.custom-select-wrap.open').forEach(w => w.classList.remove('open')));

window.addEventListener('DOMContentLoaded', () => {
  initCustomSelects();
  if (typeof initPlayerEvents === "function") initPlayerEvents();
  callRust("get_config");
  callRust("get_catalog");
  callRust("library_list");
  loadCategory('ongoing', document.querySelector('.cat-pill.active'));
});
