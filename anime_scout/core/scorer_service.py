"""Anime release grading, PTS calculation, and RuTracker search service."""

from dataclasses import dataclass, field
import re
from typing import Any

from rutracker_parser.models import RutrackerTopic, ParsedMediaInfo
from rutracker_parser.scorer import score_release


@dataclass
class ScoredRelease:
    """Standardized scored release for UI presentation and torrent dispatch."""

    topic_id: str
    title: str
    pts: int
    raw_score: float
    resolution: str
    codec: str
    voiceovers: list[str] = field(default_factory=list)
    size_bytes: int = 0
    size_human: str = ""
    seeders: int = 0
    reasons: list[str] = field(default_factory=list)


def format_bytes(b: int) -> str:
    """Format bytes into human readable Russian string (e.g. 14.2 ГБ)."""
    if b >= 1024**3:
        return f"{b / (1024**3):.1f} ГБ"
    elif b >= 1024**2:
        return f"{b / (1024**2):.0f} МБ"
    return f"{b} Б"


def extract_resolution(title: str) -> str:
    """Extract standard video resolution string."""
    t = title.lower()
    if "2160p" in t or "4k" in t or "uhd" in t:
        return "4K"
    elif "1080p" in t or "1080i" in t or "full hd" in t:
        return "1080p"
    elif "720p" in t:
        return "720p"
    return "1080p"


def extract_codec(title: str) -> str:
    """Extract primary video compression codec."""
    t = title.lower()
    if "hevc" in t or "h.265" in t or "x265" in t or "265" in t:
        return "HEVC 10-bit"
    elif "avc" in t or "h.264" in t or "x264" in t or "264" in t:
        return "AVC"
    return "HEVC"


def extract_voiceovers(title: str) -> list[str]:
    """Detect popular Russian dubbing studios and audio tracks from torrent title."""
    studios: list[str] = []
    t = title.lower()

    mapping = [
        (r"studio\s*band|студийная\s*банда", "Studio Band [DUB]"),
        (r"dream\s*cast|дрим\s*каст", "Dream Cast [DUB]"),
        (r"jam\s*club|джем", "JAM Club [DUB]"),
        (r"anilibria|анилибрия", "AniLibria [DUB]"),
        (r"anidub|анидаб", "AniDub [DUB]"),
        (r"дубляж|дублированный|то\s*дубляжная", "ТО Дубляжная [DUB]"),
        (r"crunchyroll|кранч", "Crunchyroll [SUB]"),
        (r"оригинал|японский|jap|jpn|raw", "Оригинал (Японский) [RAW]"),
    ]

    for pattern, label in mapping:
        if re.search(pattern, t):
            studios.append(label)

    if not studios:
        studios = ["Многоголосый [DUB]", "Оригинал (Японский) [RAW]"]

    return studios


class AnimeReleaseService:
    """Service evaluating torrent releases and assigning PTS points."""

    def grade_release(
        self,
        title: str,
        seeders: int,
        size_bytes: int,
        topic_id: str,
    ) -> ScoredRelease:
        """Calculate quality PTS points (0-100) and structured metadata for a release."""
        topic = RutrackerTopic(
            topic_id=int(topic_id) if topic_id.isdigit() else 1,
            title=title,
            category_id=33,
            size_bytes=size_bytes,
            seeds=seeders,
            leechers=1,
            url=f"https://rutracker.org/forum/viewtopic.php?t={topic_id}",
        )

        res_str = extract_resolution(title)
        codec_str = extract_codec(title)
        vos = extract_voiceovers(title)

        is_bdrip = bool(re.search(r"\b(bd-?rip|blu-?ray)\b", title, re.IGNORECASE))
        is_webdl = bool(re.search(r"\b(web-?dl|web-?rip)\b", title, re.IGNORECASE))
        is_tvrip = bool(re.search(r"\b(tv-?rip|hdtv)\b", title, re.IGNORECASE))

        media_info = ParsedMediaInfo(
            resolution=res_str,
            video_codec=codec_str,
            audio_tracks=vos,
            is_bdrip=is_bdrip,
            is_webdl=is_webdl,
            is_tvrip=is_tvrip,
        )

        scored = score_release(topic, media_info)
        raw_score = scored.score

        # Normalize score to 0..100 PTS scale for clear user UI display
        if raw_score <= 0:
            pts = 60
        else:
            # Typical raw scores are 30..80, map nicely to 70..98
            pts = min(98, max(65, int(65 + (raw_score * 0.45))))

        # Bonus for FLAC / Multi-audio
        if "flac" in title.lower():
            pts = min(99, pts + 3)
        if len(vos) >= 3:
            pts = min(99, pts + 2)

        return ScoredRelease(
            topic_id=str(topic_id),
            title=title,
            pts=pts,
            raw_score=raw_score,
            resolution=res_str,
            codec=codec_str,
            voiceovers=vos,
            size_bytes=size_bytes,
            size_human=format_bytes(size_bytes),
            seeders=seeders,
            reasons=scored.rank_reasons,
        )

    def rank_releases(self, releases: list[ScoredRelease]) -> list[ScoredRelease]:
        """Rank releases by PTS score descending, breaking ties with seeders count."""
        return sorted(releases, key=lambda r: (r.pts, r.seeders, r.size_bytes), reverse=True)
