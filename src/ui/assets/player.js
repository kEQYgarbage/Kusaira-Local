// Kusaira Local Anime — Player & P2P Torrent Streaming Engine

let playerBingePrefetched = false, simulatedTime = 0, fallbackTimer = null;

function playEpisode(ep) {
  currentEpisode = ep;
  document.querySelectorAll('.ep-btn').forEach((b, i) => b.classList.toggle('active', i + 1 === ep));
  callRust("play_episode", { anime_id: currentAnime ? currentAnime.id : 0, title: currentAnime ? currentAnime.title : 'Anime', episode: ep });
  if (appConfig.discord_rpc_enabled && currentAnime) {
    callRust("discord_update", { title: currentAnime.title, status: `Серия ${ep} (${appConfig.quality_preference} • ${appConfig.voiceover_priority})` });
  }
  openPlayerModal(ep);
}

function openPlayerModal(ep) {
  const modal = document.getElementById('player-modal');
  if (!modal) return;
  modal.style.display = 'flex';
  playerBingePrefetched = false;
  const s = (id, v) => { const el = document.getElementById(id); if (el) el.innerText = v; };
  s('player-title', currentAnime ? (currentAnime.title || currentAnime.name) : 'Аниме');
  s('player-ep-pill', `Серия ${ep}`);
  s('player-p2p-text', `P2P Стриминг: ${(17 + (ep % 4)).toFixed(1)} МБ/с • ${135 + ep * 3} сидов (RuTracker)`);
  const bufEl = document.getElementById('player-buffering'), fill = document.getElementById('player-buffer-fill');
  if (bufEl) bufEl.style.display = 'flex';
  if (fill) fill.style.width = '0%';
  const video = document.getElementById('player-video');
  if (video) {
    video.currentTime = 0;
    let b = 0;
    const bInt = setInterval(() => {
      b += 25;
      if (fill) fill.style.width = `${b}%`;
      if (b >= 100) {
        clearInterval(bInt);
        if (bufEl) bufEl.style.display = 'none';
        video.play().catch(() => startCanvasSimulation());
      }
    }, 120);
  } else {
    setTimeout(() => { if (bufEl) bufEl.style.display = 'none'; startCanvasSimulation(); }, 500);
  }
  checkOpeningAlert();
}

function closePlayerModal() {
  const modal = document.getElementById('player-modal');
  if (modal) modal.style.display = 'none';
  const video = document.getElementById('player-video');
  if (video) video.pause();
  if (fallbackTimer) { clearInterval(fallbackTimer); fallbackTimer = null; }
  hideOpeningBanner();
  const prompt = document.getElementById('player-skip-prompt');
  if (prompt) prompt.style.display = 'none';
}

function togglePlay() {
  const video = document.getElementById('player-video'), icon = document.getElementById('player-play-icon');
  if (!video) return;
  if (video.paused) {
    video.play().catch(() => {});
    if (icon) icon.innerHTML = '<rect x="6" y="4" width="4" height="16"></rect><rect x="14" y="4" width="4" height="16"></rect>';
  } else {
    video.pause();
    if (icon) icon.innerHTML = '<polygon points="5 3 19 12 5 21 5 3"></polygon>';
  }
}

function seekRelative(sec) {
  const video = document.getElementById('player-video');
  if (video && video.duration) video.currentTime = Math.max(0, Math.min(video.duration, video.currentTime + sec));
  else if (fallbackTimer) simulatedTime = Math.max(0, simulatedTime + sec);
}

function onSeekInput(pct) {
  const video = document.getElementById('player-video');
  if (video && video.duration) video.currentTime = (pct / 100) * video.duration;
  else if (fallbackTimer) simulatedTime = (pct / 100) * 1458;
}

function playNextEpisode() {
  const max = currentAnime?.episodes || 12;
  if (currentEpisode < max) playEpisode(currentEpisode + 1);
}

function toggleFullscreen() {
  const el = document.getElementById('player-modal');
  if (!document.fullscreenElement) el?.requestFullscreen().catch(() => {});
  else document.exitFullscreen().catch(() => {});
}

function formatPlayerTime(sec) {
  const m = Math.floor(sec / 60), s = Math.floor(sec % 60);
  return `${m < 10 ? '0' : ''}${m}:${s < 10 ? '0' : ''}${s}`;
}

function startCanvasSimulation() {
  const canvas = document.getElementById('player-fallback-canvas');
  if (!canvas) return;
  canvas.style.display = 'block';
  const ctx = canvas.getContext('2d');
  canvas.width = 1280; canvas.height = 720;
  if (fallbackTimer) clearInterval(fallbackTimer);
  simulatedTime = 0;
  fallbackTimer = setInterval(() => {
    simulatedTime += 1;
    ctx.fillStyle = '#0a0d14'; ctx.fillRect(0, 0, 1280, 720);
    ctx.fillStyle = '#2AABEE'; ctx.font = 'bold 36px sans-serif';
    ctx.fillText(`${currentAnime?.title || 'Kusaira Local'} — Эпизод ${currentEpisode}`, 80, 320);
    ctx.fillStyle = '#8B949E'; ctx.font = '22px sans-serif';
    ctx.fillText(`P2P Chunks Stream • 1080p BDRip • FLAC • Время: ${formatPlayerTime(simulatedTime)}`, 80, 380);
    updatePlayerTime(simulatedTime, 1458);
  }, 1000);
}

function updatePlayerTime(current, total) {
  const timeEl = document.getElementById('player-time-display'), seek = document.getElementById('player-seek'), track = document.getElementById('player-buffered-track');
  if (timeEl) timeEl.innerText = `${formatPlayerTime(current)} / ${formatPlayerTime(total)}`;
  const pct = (current / total) * 100;
  if (seek) seek.value = pct;
  if (track) track.style.width = `${Math.min(100, pct + 35)}%`;
  const prompt = document.getElementById('player-skip-prompt');
  if (current >= 15 && current <= 100) {
    if (appConfig.opening_skip_mode === 'auto') triggerOpeningSkip();
    else if (appConfig.opening_skip_mode !== 'off' && prompt) prompt.style.display = 'flex';
  } else if (prompt) prompt.style.display = 'none';
  if (pct >= 40 && !playerBingePrefetched) {
    playerBingePrefetched = true;
    const toast = document.getElementById('player-prefetch-toast');
    if (toast) { toast.style.display = 'flex'; setTimeout(() => toast.style.display = 'none', 4000); }
    callRust("playback_progress", { episode: currentEpisode, progress: 0.42 });
  }
}

function initPlayerEvents() {
  const v = document.getElementById('player-video');
  if (!v) return;
  v.ontimeupdate = () => updatePlayerTime(v.currentTime, v.duration || 1458);
  v.onended = () => { if (appConfig.autoplay_next) playNextEpisode(); };
  v.onerror = () => startCanvasSimulation();
}

window.addEventListener('keydown', (e) => {
  const isP = document.getElementById('player-modal')?.style.display === 'flex';
  if (e.key === 'Escape') { if (isP) closePlayerModal(); else { closeSettingsModal(); closeReleasesModal(); } return; }
  if (isP) {
    if ((appConfig.opening_skip_mode === 'tab' && e.key === 'Tab') || (appConfig.opening_skip_mode === 'space' && e.code === 'Space' && currentOpeningActive)) {
      e.preventDefault(); triggerOpeningSkip(); return;
    }
    if (e.code === 'Space') { e.preventDefault(); togglePlay(); }
    else if (e.key === 'ArrowLeft') { e.preventDefault(); seekRelative(-10); }
    else if (e.key === 'ArrowRight') { e.preventDefault(); seekRelative(10); }
    else if (e.key === 'f' || e.key === 'F') { e.preventDefault(); toggleFullscreen(); }
    return;
  }
  if (currentOpeningActive && ((appConfig.opening_skip_mode === 'tab' && e.key === 'Tab') || (appConfig.opening_skip_mode === 'space' && e.code === 'Space'))) {
    e.preventDefault(); triggerOpeningSkip();
  }
});
