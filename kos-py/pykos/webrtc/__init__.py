"""Defines the WebRTC pipeline."""

try:
    import aiortc
except ImportError:
    raise ImportError(
        "To use the WebRTC features, do `pip install pykos[webrtc]`. This requires Opus for audio streaming and "
        "LibVPX for video streaming. On Debian / Ubuntu these libraries can be installed with `apt install "
        "libopus-dev libvpx-dev` and on macOS with `brew install opus libvpx`."
    )
