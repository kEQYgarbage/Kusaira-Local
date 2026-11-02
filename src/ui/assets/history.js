// Kusaira Local Anime — Watch History & Resume Manager

let watchHistoryList = [];
let currentAnimeWatched = { watched_episodes: [], last_watched: null };
let lastHistorySavedTime = 0;

function formatHistoryTime(sec) {
  if (!sec || isNaN(sec)) return '0:00';
  const m = Math.floor(sec / 60), s = Math.floor(sec % 60);
  return `${m}:${s < 10 ? '0' : ''}${s}`;
}

function formatHistoryDate(timestamp) {
  if (!timestamp) return '';
  const d = new Date(timestamp * 1000);
  const now = new Date();
  const isToday = d.toDateString() === now.toDateString();
  if (isToday) {
    const h = d.getHours(), m = d.getMinutes();
    return `Сегодня в ${h < 10 ? '0' : ''}${h}:${m < 10 ? '0' : ''}${m}`;
  }
  return d.toLocaleDateString('ru-RU', { day: 'numeric', month: 'short' });
}

function renderWatchHistory(list) {
  watchHistoryList = Array.isArray(list) ? list : [];
  renderCatalogHistoryShelf();
  if (currentLibFilter === 'history') {
    renderHistoryLibraryTab();
  }
}

function renderCatalogHistoryShelf() {
  const wrap = document.getElementById('shelf-history-wrap');
  const container = document.getElementById('shelf-history');
  if (!wrap || !container) return;

  if (watchHistoryList.length === 0) {
    wrap.style.display = 'none';
    return;
  }

  wrap.style.display = 'block';
  container.innerHTML = watchHistoryList.slice(0, 10).map(item => {
    const dur = item.duration_sec > 0 ? item.duration_sec : 1440;
    const pos = Math.min(item.position_sec, dur);
    const pct = Math.min(100, Math.round((pos / dur) * 100));
    return `
      <div class="card card-history" onclick="resumeHistoryAnime(${item.anime_id}, ${item.episode}, ${item.position_sec})">
        <div class="card-poster">
          <img src="${item.poster}" onerror="this.src='https://placehold.co/300x420/1f232b/fff?text=Anime'" alt="${item.title}">
          <div class="history-progress-bar">
            <div class="history-progress-fill" style="width: ${pct}%"></div>
          </div>
          <span class="poster-badge ep-badge">Серия ${item.episode}</span>
        </div>
        <div class="card-title">${item.title}</div>
        <div class="card-meta">
          <span class="history-time-meta">${formatHistoryTime(pos)} / ${formatHistoryTime(dur)}</span>
        </div>
      </div>
    `;
  }).join('');
}

function renderHistoryLibraryTab() {
  const container = document.getElementById('shelf-library');
  const empty = document.getElementById('library-empty');
  if (!container) return;

  if (watchHistoryList.length === 0) {
    container.innerHTML = '';
    if (empty) {
      empty.style.display = 'block';
      empty.innerHTML = `
        <svg class="icon empty-icon" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"></circle><polyline points="12 6 12 12 16 14"></polyline></svg>
        <div class="empty-title">История просмотров пуста</div>
        <div class="empty-sub">Серии, которые вы смотрите, будут сохраняться здесь с точным таймкодом.</div>
      `;
    }
    return;
  }

  if (empty) empty.style.display = 'none';

  const clearBtnHtml = `
    <div class="history-actions-bar">
      <span class="history-count">Просмотрено: ${watchHistoryList.length}</span>
      <button class="btn-clear-history" onclick="clearWatchHistory()">
        <svg class="icon" viewBox="0 0 24 24"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
        <span>Очистить историю</span>
      </button>
    </div>
  `;

  const cardsHtml = watchHistoryList.map(item => {
    const dur = item.duration_sec > 0 ? item.duration_sec : 1440;
    const pos = Math.min(item.position_sec, dur);
    const pct = Math.min(100, Math.round((pos / dur) * 100));
    return `
      <div class="card card-history">
        <div class="card-poster" onclick="openDetails(${item.anime_id})">
          <img src="${item.poster}" onerror="this.src='https://placehold.co/300x420/1f232b/fff?text=Anime'" alt="${item.title}">
          <div class="history-progress-bar">
            <div class="history-progress-fill" style="width: ${pct}%"></div>
          </div>
          <span class="poster-badge ep-badge">Серия ${item.episode}</span>
        </div>
        <div class="card-title" onclick="openDetails(${item.anime_id})">${item.title}</div>
        <div class="card-meta">
          <span>${formatHistoryDate(item.watched_at)}</span>
          <span class="history-time-meta">${formatHistoryTime(pos)} / ${formatHistoryTime(dur)}</span>
        </div>
        <button class="btn-resume-card" onclick="resumeHistoryAnime(${item.anime_id}, ${item.episode}, ${item.position_sec})">
          <svg class="icon" viewBox="0 0 24 24"><polygon points="5 3 19 12 5 21 5 3"></polygon></svg>
          <span>Продолжить</span>
        </button>
      </div>
    `;
  }).join('');

  container.innerHTML = clearBtnHtml + '<div class="history-grid">' + cardsHtml + '</div>';
}

function setAnimeWatchedHistory(payload) {
  if (!payload || !currentAnime || Number(currentAnime.id) !== Number(payload.anime_id)) return;
  currentAnimeWatched = payload;

  const watchedSet = new Set(payload.watched_episodes || []);
  document.querySelectorAll('#episodes-grid .ep-btn').forEach(btn => {
    const epNum = Number(btn.innerText.trim());
    if (watchedSet.has(epNum)) {
      btn.classList.add('watched');
      btn.title = `Серия ${epNum} (просмотрена)`;
    } else {
      btn.classList.remove('watched');
      btn.title = `Серия ${epNum}`;
    }
  });

  const resumeWrap = document.getElementById('detail-resume-wrap');
  if (resumeWrap) {
    if (payload.last_watched && payload.last_watched.episode) {
      const lw = payload.last_watched;
      resumeWrap.style.display = 'block';
      resumeWrap.innerHTML = `
        <button class="btn-resume-main" onclick="resumeEpisode(${lw.episode}, ${lw.position_sec})">
          <svg class="icon" viewBox="0 0 24 24"><polygon points="5 3 19 12 5 21 5 3"></polygon></svg>
          <span>Продолжить просмотр: Серия ${lw.episode} (${formatHistoryTime(lw.position_sec)})</span>
        </button>
      `;
    } else {
      resumeWrap.style.display = 'none';
      resumeWrap.innerHTML = '';
    }
  }
}

async function resumeHistoryAnime(animeId, ep, posSec) {
  await openDetails(animeId);
  resumeEpisode(ep, posSec);
}

function resumeEpisode(ep, posSec) {
  playEpisode(ep, posSec);
}

function clearWatchHistory() {
  callRust("clear_history");
}

function saveCurrentWatchProgress(currentSec, totalSec) {
  if (!currentAnime || !currentEpisode) return;
  const now = Math.floor(Date.now() / 1000);
  lastHistorySavedTime = now;

  const entry = {
    anime_id: Number(currentAnime.id),
    episode: currentEpisode,
    title: currentAnime.title || currentAnime.name || 'Аниме',
    name: currentAnime.name || currentAnime.title || '',
    poster: currentAnime.poster || '',
    position_sec: Math.max(0, currentSec || 0),
    duration_sec: Math.max(1, totalSec || 1440),
    watched_at: now,
  };

  callRust("record_history", { history_entry: entry });

  if (!currentAnimeWatched.watched_episodes) currentAnimeWatched.watched_episodes = [];
  if (!currentAnimeWatched.watched_episodes.includes(currentEpisode)) {
    currentAnimeWatched.watched_episodes.push(currentEpisode);
  }
  currentAnimeWatched.last_watched = entry;

  const btn = Array.from(document.querySelectorAll('#episodes-grid .ep-btn')).find(b => Number(b.innerText.trim()) === currentEpisode);
  if (btn) btn.classList.add('watched');
}
