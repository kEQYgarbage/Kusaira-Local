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
      <div class="card-meta">★ ${a.score || '8.5'} • ${a.episodes || 12} эп.</div>
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

function searchAnime(q) {
  const query = q.trim().toLowerCase(), wrap = document.getElementById('shelf-search-wrap'), shelves = document.getElementById('catalog-shelves');
  if (!query) {
    if (wrap) wrap.style.display = 'none';
    if (shelves) shelves.style.display = 'block';
    return;
  }
  const local = Array.from(animeDatabase.values()).filter(a => (a.title && a.title.toLowerCase().includes(query)) || (a.name && a.name.toLowerCase().includes(query)));
  renderSearchResults(local);
  callRust("search", { query: q });
}

function openDetails(animeId) {
  const idNum = Number(animeId);
  if (animeDatabase.has(idNum)) setDetails(animeDatabase.get(idNum));
  callRust("get_details", { id: idNum });
  callRust("get_related", { id: idNum });
  navTo('detail');
}

function setDetails(anime) {
  if (!anime) return;
  currentAnime = anime;
  animeDatabase.set(Number(anime.id), anime);

  const setEl = (id, val) => { const el = document.getElementById(id); if (el) el.innerText = val; };
  setEl('detail-title', anime.title || '');
  setEl('detail-subtitle', anime.name || '');
  setEl('detail-score', `★ ${anime.score || '8.8'}`);
  setEl('detail-pts', `${anime.pts || 96} PTS`);
  setEl('detail-active-pts', `${anime.pts || 96} PTS`);
  setEl('detail-release', anime.release || '1080p BDRip HEVC FLAC');
  setEl('detail-desc', anime.description || '');

  const imgEl = document.getElementById('detail-poster-img');
  if (imgEl) imgEl.src = anime.poster || '';
  updateDetailLibButton(anime.id);

  const total = anime.episodes || 12, grid = document.getElementById('episodes-grid');
  if (grid) {
    grid.innerHTML = Array.from({length: total}, (_, i) => i + 1).map(n => `
      <button class="ep-btn ${n === currentEpisode ? 'active' : ''}" onclick="playEpisode(${n})">${n}</button>
    `).join('');
  }
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
    if (icon) icon.innerText = '➕';
    if (text) text.innerText = 'В библиотеку';
    return;
  }
  const map = { watching: '👁️ Смотрю', planned: '⏳ В планах', completed: '✅ Просмотрено', favorite: '⭐ В избранном' };
  if (text) text.innerText = map[entry.status] || 'В библиотеке';
  if (icon) icon.innerText = '✓';
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
  const labelMap = { watching: '👁️ Смотрю', planned: '⏳ В планах', completed: '✅ Просмотрено', favorite: '⭐ Избранное' };
  container.innerHTML = filtered.map(a => `
    <div class="card" onclick="openDetails(${a.anime_id})">
      <div class="card-poster">
        <img src="${a.poster}" onerror="this.src='https://placehold.co/300x420/1f232b/fff?text=Anime'" alt="${a.title}">
        <span class="poster-badge">${labelMap[a.status] || a.status}</span>
      </div>
      <div class="card-title">${a.title}</div>
      <div class="card-meta">★ ${a.score} • ${a.episodes} эп.</div>
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
  if (releases && releases.length > 0 && !selectedReleaseTopicId) selectedReleaseTopicId = releases[0].topic_id;
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
        ${selectedReleaseTopicId === r.topic_id ? '✓ Активен' : 'Выбрать'}
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
  callRust("player_action", { action: "skip_forward_85s" });
}

/* Theming & Density */
function previewTheme(th) { document.body.setAttribute('data-theme', th); }
function previewDensity(d) { document.body.setAttribute('data-density', d); }

/* Settings Modal Multi-tab */
function openSettingsModal() {
  const modal = document.getElementById('settings-modal');
  if (modal) modal.style.display = 'flex';
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
  previewTheme(appConfig.theme);
  previewDensity(appConfig.grid_density);
  const setVal = (id, val) => { const el = document.getElementById(id); if (el) el.value = val; };
  setVal('set-theme', appConfig.theme);
  setVal('set-density', appConfig.grid_density);
  setVal('set-op-skip', appConfig.opening_skip_mode);
  setVal('set-seek-step', appConfig.seek_step_sec);
  setVal('set-gc-max-gb', appConfig.gc_max_disk_gb);
  setVal('set-gc-policy', appConfig.gc_policy);
  setVal('set-voiceover', appConfig.voiceover_priority);
  setVal('set-quality', appConfig.quality_preference);
  setVal('set-hwaccel', appConfig.hw_accel);

  const setCheck = (id, val) => { const el = document.getElementById(id); if (el) el.checked = !!val; };
  setCheck('set-autoplay', appConfig.autoplay_next);
  setCheck('set-resume', appConfig.resume_playback);
  setCheck('set-skip-ed', appConfig.skip_endings);
  setCheck('set-gc-exit', appConfig.gc_on_exit);
  setCheck('set-discord-rpc', appConfig.discord_rpc_enabled);
}

function saveSettings() {
  appConfig.theme = document.getElementById('set-theme').value;
  appConfig.grid_density = document.getElementById('set-density').value;
  appConfig.opening_skip_mode = document.getElementById('set-op-skip').value;
  appConfig.seek_step_sec = Number(document.getElementById('set-seek-step').value);
  appConfig.gc_max_disk_gb = Number(document.getElementById('set-gc-max-gb').value);
  appConfig.gc_policy = document.getElementById('set-gc-policy').value;
  appConfig.voiceover_priority = document.getElementById('set-voiceover').value;
  appConfig.quality_preference = document.getElementById('set-quality').value;
  appConfig.hw_accel = document.getElementById('set-hwaccel').value;
  appConfig.autoplay_next = document.getElementById('set-autoplay').checked;
  appConfig.resume_playback = document.getElementById('set-resume').checked;
  appConfig.skip_endings = document.getElementById('set-skip-ed').checked;
  appConfig.gc_on_exit = document.getElementById('set-gc-exit').checked;
  appConfig.discord_rpc_enabled = document.getElementById('set-discord-rpc').checked;

  previewTheme(appConfig.theme);
  previewDensity(appConfig.grid_density);
  callRust("save_config", { config: appConfig });
  closeSettingsModal();
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

/* Player & Episode */
function playEpisode(ep) {
  currentEpisode = ep;
  document.querySelectorAll('.ep-btn').forEach((b, i) => b.classList.toggle('active', i + 1 === ep));
  callRust("play_episode", { anime_id: currentAnime ? currentAnime.id : 0, episode: ep });
  if (appConfig.discord_rpc_enabled && currentAnime) {
    callRust("discord_update", { title: currentAnime.title, status: `Серия ${ep} (${appConfig.quality_preference} • ${appConfig.voiceover_priority})` });
  }
  checkOpeningAlert();
}

window.addEventListener('keydown', (e) => {
  if (currentOpeningActive) {
    if ((appConfig.opening_skip_mode === 'tab' && e.key === 'Tab') || (appConfig.opening_skip_mode === 'space' && e.code === 'Space')) {
      e.preventDefault();
      triggerOpeningSkip();
    }
  }
});

window.addEventListener('DOMContentLoaded', () => {
  callRust("get_config");
  callRust("get_catalog");
  callRust("library_list");
});
