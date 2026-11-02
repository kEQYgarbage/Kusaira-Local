// Kusaira Local Anime — Player & P2P Streaming Engine

let playerBingePrefetched = false, activePlayerSource = 'stream', lastPlayerProgressSaveTime = 0;
const P2P_STREAM_URL = "https://cdn.plyr.io/static/demo/View_From_A_Blue_Moon_Trailer-720p.mp4";
let currentStreamUrl = P2P_STREAM_URL;

function onStreamReady(data) {
  if (!data) return;
  if (data.stream_url) {
    currentStreamUrl = data.stream_url;
    const v = document.getElementById('player-video');
    if (v && activePlayerSource === 'stream') {
      v.src = data.stream_url;
      v.play().catch(() => {});
    }
  }
  const p2pText = document.getElementById('player-p2p-text');
  if (p2pText) {
    p2pText.innerText = `P2P Стриминг: ${data.speed_mbps || 18.2} МБ/с • ${data.seeds || 58} сидов (Локальный rqbit)`;
  }
}

function playEpisode(ep, resumePosSec = null) {
  currentEpisode = ep;
  document.querySelectorAll('.ep-btn').forEach((b, i) => b.classList.toggle('active', i + 1 === ep));
  callRust("play_episode", {
    anime_id: currentAnime ? currentAnime.id : 0,
    title: currentAnime ? (currentAnime.title || currentAnime.name) : 'Anime',
    topic_id: selectedReleaseTopicId || 'default',
    episode: ep,
  });
  if (appConfig.discord_rpc_enabled && currentAnime) {
    callRust("discord_update", { title: currentAnime.title, status: `Серия ${ep} (${appConfig.quality_preference} • ${appConfig.voiceover_priority})` });
  }
  openPlayerModal(ep, resumePosSec);
}

function openPlayerModal(ep, resumePosSec = null) {
  const modal = document.getElementById('player-modal');
  if (!modal) return;
  modal.style.display = 'flex';
  playerBingePrefetched = false;

  const s = (id, v) => { const el = document.getElementById(id); if (el) el.innerText = v; };
  s('player-title', currentAnime ? (currentAnime.title || currentAnime.name) : 'Аниме');
  s('player-ep-pill', `Серия ${ep}`);
  s('player-p2p-text', `P2P Стриминг: ${(17 + (ep % 4)).toFixed(1)} МБ/с • ${135 + ep * 3} сидов (RuTracker)`);

  const pvBtn = document.getElementById('btn-src-pv');
  if (pvBtn) pvBtn.style.display = (currentAnime?.videos && currentAnime.videos.length > 0) ? 'inline-flex' : 'none';

  switchPlayerSource('stream');

  const bufEl = document.getElementById('player-buffering'), fill = document.getElementById('player-buffer-fill');
  if (bufEl) bufEl.style.display = 'flex';
  if (fill) fill.style.width = '0%';

  const video = document.getElementById('player-video');
  if (video) {
    const shouldResume = appConfig.resume_playback && typeof resumePosSec === 'number' && resumePosSec > 5;
    video.currentTime = shouldResume ? resumePosSec : 0;
    let b = 0;
    const bInt = setInterval(() => {
      b += 30;
      if (fill) fill.style.width = `${b}%`;
      if (b >= 100) {
        clearInterval(bInt);
        if (bufEl) bufEl.style.display = 'none';
        if (shouldResume && Math.abs(video.currentTime - resumePosSec) > 1) {
          video.currentTime = resumePosSec;
        }
        video.play().catch(() => {
          video.muted = true;
          updateMuteIcon(true);
          video.play().catch(() => {});
        });
      }
    }, 100);
  } else {
    setTimeout(() => { if (bufEl) bufEl.style.display = 'none'; }, 400);
  }
  checkOpeningAlert();
}

function switchPlayerSource(src) {
  activePlayerSource = src;
  document.querySelectorAll('.player-src-btn').forEach(b => b.classList.remove('active'));
  const btn = document.getElementById(`btn-src-${src}`);
  if (btn) btn.classList.add('active');

  const video = document.getElementById('player-video'), iframe = document.getElementById('player-iframe');
  if (src === 'stream') {
    if (iframe) { iframe.style.display = 'none'; iframe.src = ""; }
    if (video) {
      video.style.display = 'block';
      if (!video.src || (!video.src.includes('127.0.0.1') && !video.src.includes('plyr.io'))) video.src = currentStreamUrl;
      video.play().catch(() => {});
    }
  } else if (src === 'pv') {
    if (video) { video.pause(); video.style.display = 'none'; }
    if (iframe && currentAnime?.videos?.length) {
      iframe.style.display = 'block';
      let raw = currentAnime.videos[0].player_url || currentAnime.videos[0].url || '';
      if (raw.includes('youtu.be/')) raw = `https://www.youtube-nocookie.com/embed/${raw.split('youtu.be/')[1].split('?')[0]}?autoplay=1`;
      else if (raw.includes('youtube.com/embed/')) raw = raw.replace('http://', 'https://').replace('youtube.com', 'youtube-nocookie.com') + '?autoplay=1';
      iframe.src = raw;
    }
  }
}

function openExternalVlc() {
  callRust("open_vlc", { episode: currentEpisode, query: currentStreamUrl });
}

function closePlayerModal() {
  const modal = document.getElementById('player-modal');
  if (modal) modal.style.display = 'none';
  const video = document.getElementById('player-video'), iframe = document.getElementById('player-iframe');
  if (video) {
    if (typeof saveCurrentWatchProgress === 'function' && video.currentTime > 0) {
      saveCurrentWatchProgress(video.currentTime, video.duration || 1440);
    }
    video.pause();
  }
  if (iframe) { iframe.src = ""; iframe.style.display = 'none'; }
  hideOpeningBanner();
  const prompt = document.getElementById('player-skip-prompt');
  if (prompt) prompt.style.display = 'none';
  callRust("get_history");
}

function togglePlay() {
  const video = document.getElementById('player-video');
  if (!video) return;
  if (video.paused) video.play().catch(() => {});
  else video.pause();
}

function toggleMute() {
  const video = document.getElementById('player-video');
  if (!video) return;
  video.muted = !video.muted;
  updateMuteIcon(video.muted);
}

function updateMuteIcon(isMuted) {
  const icon = document.getElementById('player-mute-icon');
  if (!icon) return;
  icon.innerHTML = isMuted
    ? '<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon><line x1="23" y1="9" x2="17" y2="15"></line><line x1="17" y1="9" x2="23" y2="15"></line>'
    : '<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon><path d="M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07"></path>';
}

function seekRelative(sec) {
  const video = document.getElementById('player-video');
  if (video && video.duration) video.currentTime = Math.max(0, Math.min(video.duration, video.currentTime + sec));
}

function onSeekInput(pct) {
  const video = document.getElementById('player-video');
  if (video && video.duration) video.currentTime = (pct / 100) * video.duration;
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

function updatePlayerTime(current, total) {
  const timeEl = document.getElementById('player-time-display'), seek = document.getElementById('player-seek'), track = document.getElementById('player-buffered-track');
  if (timeEl) timeEl.innerText = `${formatPlayerTime(current)} / ${formatPlayerTime(total)}`;
  const pct = (current / total) * 100;
  if (seek) seek.value = pct;
  if (track) track.style.width = `${Math.min(100, pct + 35)}%`;

  const prompt = document.getElementById('player-skip-prompt');
  if (current >= 15 && current <= 95) {
    if (appConfig.opening_skip_mode === 'auto') triggerOpeningSkip();
    else if (appConfig.opening_skip_mode !== 'off' && prompt) prompt.style.display = 'flex';
  } else if (prompt) prompt.style.display = 'none';

  if (pct >= 40 && !playerBingePrefetched) {
    playerBingePrefetched = true;
    const toast = document.getElementById('player-prefetch-toast');
    if (toast) { toast.style.display = 'flex'; setTimeout(() => toast.style.display = 'none', 4000); }
    callRust("playback_progress", { episode: currentEpisode, progress: 0.42 });
  }

  if (typeof saveCurrentWatchProgress === 'function' && Math.abs(current - lastPlayerProgressSaveTime) >= 8) {
    lastPlayerProgressSaveTime = current;
    saveCurrentWatchProgress(current, total);
  }
}

function triggerOpeningSkip() {
  hideOpeningBanner();
  const prompt = document.getElementById('player-skip-prompt');
  if (prompt) prompt.style.display = 'none';
  const v = document.getElementById('player-video');
  if (v && v.currentTime < 95) v.currentTime = 95;
  callRust("player_action", { action: "skip_forward_85s" });
}

function initPlayerEvents() {
  const v = document.getElementById('player-video');
  if (!v) return;
  v.ontimeupdate = () => updatePlayerTime(v.currentTime, v.duration || 1458);
  v.onended = () => { if (appConfig.autoplay_next) playNextEpisode(); };
  v.onplay = () => {
    const icon = document.getElementById('player-play-icon');
    if (icon) icon.innerHTML = '<rect x="6" y="4" width="4" height="16"></rect><rect x="14" y="4" width="4" height="16"></rect>';
  };
  v.onpause = () => {
    const icon = document.getElementById('player-play-icon');
    if (icon) icon.innerHTML = '<polygon points="5 3 19 12 5 21 5 3"></polygon>';
  };
}

window.addEventListener('keydown', (e) => {
  const isP = document.getElementById('player-modal')?.style.display === 'flex';
  if (e.key === 'Escape') { if (isP) closePlayerModal(); else { closeSettingsModal(); closeReleasesModal(); } return; }
  if (isP) {
    if ((appConfig.opening_skip_mode === 'tab' && e.key === 'Tab') || (appConfig.opening_skip_mode === 'space' && e.code === 'Space' && currentOpeningActive)) {
      e.preventDefault(); triggerOpeningSkip(); return;
    }
    if (e.code === 'Space') { e.preventDefault(); togglePlay(); }
    else if (e.key === 'm' || e.key === 'M') { e.preventDefault(); toggleMute(); }
    else if (e.key === 'ArrowLeft') { e.preventDefault(); seekRelative(-10); }
    else if (e.key === 'ArrowRight') { e.preventDefault(); seekRelative(10); }
    else if (e.key === 'f' || e.key === 'F') { e.preventDefault(); toggleFullscreen(); }
    return;
  }
  if (currentOpeningActive && ((appConfig.opening_skip_mode === 'tab' && e.key === 'Tab') || (appConfig.opening_skip_mode === 'space' && e.code === 'Space'))) {
    e.preventDefault(); triggerOpeningSkip();
  }
});
