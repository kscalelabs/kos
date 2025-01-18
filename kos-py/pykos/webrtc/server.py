"""Defines a simple WebRTC server using aiortc."""

import asyncio
import json
import logging
import platform
from typing import Any, Coroutine, List, Set

from aiohttp import web
from aiortc import MediaStreamTrack, RTCPeerConnection, RTCSessionDescription
from aiortc.contrib.media import MediaPlayer

logger = logging.getLogger(__name__)


def get_platform_media_config() -> tuple[str, str]:
    """Get the appropriate media source format for the current platform.

    Returns:
        Tuple of (device_string, format_string)
    """
    system = platform.system().lower()
    if system == "darwin":  # MacOS
        return "default:none", "avfoundation"
    elif system == "linux":
        return "/dev/video0", "v4l2"  # Video4Linux2
    else:
        logger.warning("Unsupported platform %s, falling back to test pattern", system)
        return "color:blue", "lavfi"


async def create_local_tracks() -> tuple[MediaStreamTrack | None, MediaStreamTrack | None]:
    """Create local video and audio tracks from webcam/microphone."""
    device_string, format_string = get_platform_media_config()

    try:
        # Try to use the default webcam and microphone
        player = MediaPlayer(
            device_string,
            format=format_string,
            options={"framerate": "30", "video_size": "640x480"},
        )
        if not player.video:
            raise RuntimeError("No video device found")
        return player.video, player.audio

    except Exception as e:
        logger.warning("Could not open webcam/microphone: %s", e)
        logger.warning("Using test sources instead")
        player = MediaPlayer(
            "color:blue",
            format="lavfi",
            options={"framerate": "30", "video_size": "640x480"},
        )
        return player.video, None


pcs: Set[RTCPeerConnection] = set()


async def offer(request: web.Request) -> web.Response:
    """Handle incoming WebRTC offers."""
    params = await request.json()
    offer = RTCSessionDescription(sdp=params["sdp"], type=params["type"])

    pc = RTCPeerConnection()
    pcs.add(pc)

    # Create local tracks
    video_sender, audio_sender = await create_local_tracks()
    if video_sender:
        pc.addTrack(video_sender)
    if audio_sender:
        pc.addTrack(audio_sender)

    @pc.on("track")
    def on_track(track: MediaStreamTrack) -> None:
        print(f"Receiving {track.kind} track from client")

    @pc.on("connectionstatechange")
    async def on_connectionstatechange() -> None:
        print(f"Connection state is {pc.connectionState}")
        if pc.connectionState == "failed":
            await pc.close()
            pcs.discard(pc)

    await pc.setRemoteDescription(offer)
    answer = await pc.createAnswer()
    await pc.setLocalDescription(answer)

    return web.Response(
        content_type="application/json",
        text=json.dumps({"sdp": pc.localDescription.sdp, "type": pc.localDescription.type}),
    )


async def on_shutdown(app: web.Application) -> None:
    """Close all peer connections when shutting down."""
    coros: List[Coroutine[Any, Any, None]] = [pc.close() for pc in pcs]
    await asyncio.gather(*coros)
    pcs.clear()


async def init_server() -> web.Application:
    """Initialize and run the WebRTC server."""
    app = web.Application()
    app.on_shutdown.append(on_shutdown)
    app.router.add_post("/offer", offer)
    return app


def main() -> None:
    """Run the WebRTC server."""
    app = init_server()
    web.run_app(app, host="0.0.0.0", port=8080)


if __name__ == "__main__":
    # python -m pykos.webrtc.server
    main()
