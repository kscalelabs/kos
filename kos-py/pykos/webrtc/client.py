"""Defines a simple WebRTC client using aiortc."""

import asyncio
import logging
import platform
from abc import ABC, abstractmethod

import aiohttp
import cv2
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
        return "color:red", "lavfi"


class WebRTCClient(ABC):
    """Abstract base class for WebRTC clients."""

    def __init__(self, server_url: str = "http://localhost:8080") -> None:
        """Initialize the WebRTC client.

        Args:
            server_url: URL of the WebRTC signaling server
        """
        self.server_url = server_url
        self.pc = RTCPeerConnection()
        self._tracks_added = asyncio.Event()

    async def create_local_tracks(self) -> tuple[MediaStreamTrack | None, MediaStreamTrack | None]:
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
                "color:red",
                format="lavfi",
                options={"framerate": "30", "video_size": "640x480"},
            )
            return player.video, None

    async def _setup_tracks(self) -> None:
        """Set up track handling."""
        # Add local tracks
        video, audio = await self.create_local_tracks()
        if video:
            self.pc.addTrack(video)
        if audio:
            self.pc.addTrack(audio)

        @self.pc.on("track")
        async def on_track(track: MediaStreamTrack) -> None:
            print(f"Receiving {track.kind} track")
            if track.kind == "video":
                await self.handle_video_track(track)
            elif track.kind == "audio":
                await self.handle_audio_track(track)

        # Signal that tracks have been added
        self._tracks_added.set()

    @abstractmethod
    async def handle_video_track(self, track: MediaStreamTrack) -> None:
        """Handle incoming video track.

        Args:
            track: The video MediaStreamTrack to handle
        """
        pass

    @abstractmethod
    async def handle_audio_track(self, track: MediaStreamTrack) -> None:
        """Handle incoming audio track.

        Args:
            track: The audio MediaStreamTrack to handle
        """
        pass

    async def connect(self) -> None:
        """Establish WebRTC connection with the server."""
        # Setup tracks and wait for them to be added
        await self._setup_tracks()
        await self._tracks_added.wait()

        # Create offer
        await self.pc.setLocalDescription(await self.pc.createOffer())

        # Send offer to server
        async with aiohttp.ClientSession() as session:
            async with session.post(
                f"{self.server_url}/offer",
                json={
                    "sdp": self.pc.localDescription.sdp,
                    "type": self.pc.localDescription.type,
                },
            ) as resp:
                answer = await resp.json()
                await self.pc.setRemoteDescription(RTCSessionDescription(sdp=answer["sdp"], type=answer["type"]))

        # Wait for connection to establish
        while self.pc.connectionState != "connected":
            await asyncio.sleep(0.1)

    async def disconnect(self) -> None:
        """Disconnect from the WebRTC server."""
        await self.pc.close()

    @abstractmethod
    async def run(self) -> None:
        """Run the client. Must be implemented by subclasses."""
        pass


class OpenCVClient(WebRTCClient):
    """Default WebRTC client implementation using OpenCV for video display."""

    def __init__(self, server_url: str = "http://localhost:8080") -> None:
        """Initialize the OpenCV client.

        Args:
            server_url: URL of the WebRTC signaling server
        """
        super().__init__(server_url)
        # Create windows for local and remote video
        self.remote_window = "Remote Video"
        self.local_window = "Local Video"
        cv2.namedWindow(self.remote_window)
        cv2.namedWindow(self.local_window)

        # Store local track for display
        self.local_track: MediaStreamTrack | None = None

    async def create_local_tracks(self) -> tuple[MediaStreamTrack | None, MediaStreamTrack | None]:
        """Create and store local tracks."""
        video, audio = await super().create_local_tracks()
        self.local_track = video
        return video, audio

    async def handle_video_track(self, track: MediaStreamTrack) -> None:
        """Handle incoming video track by displaying frames using OpenCV.

        Args:
            track: The video MediaStreamTrack to display
        """

        async def video_display() -> None:
            while True:
                try:
                    # Display remote video
                    frame = await track.recv()
                    img = frame.to_ndarray(format="bgr24")
                    cv2.imshow(self.remote_window, img)

                    # Display local video if available
                    if self.local_track:
                        local_frame = await self.local_track.recv()
                        local_img = local_frame.to_ndarray(format="bgr24")
                        cv2.imshow(self.local_window, local_img)

                    cv2.waitKey(1)
                except Exception as e:
                    print(f"Error displaying frame: {e}")
                    break

        asyncio.ensure_future(video_display())

    async def handle_audio_track(self, track: MediaStreamTrack) -> None:
        """Handle incoming audio track (currently just prints a message).

        Args:
            track: The audio MediaStreamTrack to handle
        """
        print("Audio track received - not implemented in OpenCV client")

    async def run(self) -> None:
        """Run the OpenCV client."""
        try:
            await self.connect()
            # Keep connection alive
            await asyncio.sleep(3600)  # Run for 1 hour
        except KeyboardInterrupt:
            pass
        finally:
            cv2.destroyWindow(self.remote_window)
            cv2.destroyWindow(self.local_window)
            await self.disconnect()


async def run_default_client(server_url: str = "http://localhost:8080") -> None:
    """Run the default OpenCV client implementation.

    Args:
        server_url: URL of the WebRTC signaling server
    """
    client = OpenCVClient(server_url)
    await client.run()


def main() -> None:
    """Run the WebRTC client."""
    asyncio.run(run_default_client())


if __name__ == "__main__":
    # python -m pykos.webrtc.client
    main()
