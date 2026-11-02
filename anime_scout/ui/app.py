"""FastAPI application factory and local desktop controller."""

import os
from typing import Any
from fastapi import FastAPI, HTTPException
from fastapi.responses import HTMLResponse
from pydantic import BaseModel

from anime_scout.core.scorer_service import AnimeReleaseService
from anime_scout.player.vlc_engine import VlcEngine, PlaybackState
from anime_scout.ui.kusaira_theme import DEFAULT_ANIME_CATALOG


class PlayRequest(BaseModel):
    anime_id: str
    episode: int
    release_id: str | None = None


class PlayerAction(BaseModel):
    action: str  # 'toggle_pause', 'seek', 'next'
    value: Any | None = None


def create_app() -> FastAPI:
    """Create and configure FastAPI application for Kusaira Desktop."""
    app = FastAPI(title="Kusaira Desktop", version="1.0.0")
    
    # Core services
    scorer_service = AnimeReleaseService()
    # Try initializing VLC engine if libvlc is available
    try:
        vlc_engine = VlcEngine()
    except Exception:
        vlc_engine = None

    template_path = os.path.join(os.path.dirname(__file__), "templates", "index.html")

    @app.get("/", response_class=HTMLResponse)
    async def index():
        if os.path.exists(template_path):
            with open(template_path, "r", encoding="utf-8") as f:
                return HTMLResponse(f.read())
        return HTMLResponse("<h1>Kusaira Desktop</h1>")

    @app.get("/api/search")
    async def search(q: str = ""):
        # Search catalog and rank releases
        q_lower = q.lower()
        matched = [a for a in DEFAULT_ANIME_CATALOG if q_lower in a["title"].lower() or q_lower in a.get("subtitle", "").lower()]
        if not matched:
            matched = DEFAULT_ANIME_CATALOG
        return {"results": matched, "query": q}

    @app.get("/api/anime/{anime_id}/details")
    async def get_anime_details(anime_id: str):
        for a in DEFAULT_ANIME_CATALOG:
            if str(a["id"]) == str(anime_id):
                return a
        raise HTTPException(status_code=404, detail="Anime not found")

    @app.post("/api/play")
    async def play_episode(req: PlayRequest):
        if vlc_engine:
            try:
                # If local file exists, play it, else simulate playback
                pass
            except Exception as e:
                return {"status": "error", "message": str(e)}
        return {"status": "ok", "playing_episode": req.episode, "anime_id": req.anime_id}

    @app.post("/api/player/action")
    async def handle_player_action(action: PlayerAction):
        if not vlc_engine:
            return {"status": "ok", "state": "playing"}

        if action.action == "toggle_pause":
            vlc_engine.pause()
        elif action.action == "seek" and action.value is not None:
            vlc_engine.set_time(float(action.value))

        return {"status": "ok", "state": vlc_engine.state.value}

    @app.get("/api/player/status")
    async def player_status():
        if not vlc_engine:
            return {"state": "stopped", "time": 0.0, "duration": 1450.0, "speed": 18.2}
        return {
            "state": vlc_engine.state.value,
            "time": vlc_engine.get_time(),
            "duration": vlc_engine.get_duration(),
            "is_playing": vlc_engine.is_playing(),
        }

    return app


def main():
    """Launch Kusaira Desktop local server and open in standalone desktop window."""
    import time
    import threading
    import uvicorn
    from anime_scout.ui.window import launch_desktop_window

    app = create_app()
    port = 8765
    url = f"http://127.0.0.1:{port}"

    config = uvicorn.Config(app, host="127.0.0.1", port=port, log_level="warning")
    server = uvicorn.Server(config)

    server_thread = threading.Thread(target=server.run, daemon=True)
    server_thread.start()

    # Wait briefly for server initialization
    time.sleep(0.5)

    # Launch standalone application window
    proc = launch_desktop_window(url)
    if proc:
        try:
            proc.wait()
        except KeyboardInterrupt:
            proc.terminate()
        finally:
            server.should_exit = True
            server_thread.join(timeout=2.0)
    else:
        try:
            while server_thread.is_alive():
                time.sleep(0.5)
        except KeyboardInterrupt:
            server.should_exit = True
            server_thread.join(timeout=2.0)


if __name__ == "__main__":
    main()
