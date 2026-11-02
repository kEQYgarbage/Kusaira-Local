// Kusaira Local Anime — Desktop Frontend Controller

let currentAnime = null;
let currentEpisode = 1;

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
  if (view === 'catalog') {
    document.getElementById('view-catalog').classList.add('active');
    document.getElementById('nav-btn-cat').classList.add('active');
  } else {
    document.getElementById('view-detail').classList.add('active');
    document.getElementById('nav-btn-det').classList.add('active');
  }
}

function renderShelf(containerId, items) {
  const el = document.getElementById(containerId);
  if (!el) return;
  el.innerHTML = items.map(a => `
    <div class="card" onclick="openDetails('${a.id}')">
      <div class="card-poster">
        <img src="${a.poster}" onerror="this.src='https://placehold.co/300x420/1f232b/fff?text=Anime'" alt="${a.title}">
        <span class="poster-badge">1080p • <span class="pts-badge">${a.pts || 95} PTS</span></span>
      </div>
      <div class="card-title">${a.title}</div>
      <div class="card-meta">★ ${a.score || '8.5'} • ${a.episodes || 12} эп.</div>
    </div>
  `).join('');
}

function openDetails(animeId) {
  callRust("get_details", { id: animeId });
  navTo('detail');
}

function setDetails(anime) {
  currentAnime = anime;
  document.getElementById('detail-title').innerText = anime.title;
  document.getElementById('detail-subtitle').innerText = anime.name || '';
  document.getElementById('detail-poster-img').src = anime.poster;
  
  const total = anime.episodes || 12;
  const grid = document.getElementById('episodes-grid');
  grid.innerHTML = Array.from({length: total}, (_, i) => i + 1).map(n => `
    <button class="ep-btn ${n === 1 ? 'active' : ''}" onclick="playEpisode(${n})">${n}</button>
  `).join('');
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

function searchAnime(q) {
  if (q.trim().length > 1) {
    callRust("search", { query: q });
  }
}

// Initial catalog request
window.addEventListener('DOMContentLoaded', () => {
  callRust("get_catalog");
});
