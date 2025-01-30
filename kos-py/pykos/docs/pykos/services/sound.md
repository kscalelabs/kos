[  pykos.services](../services.html)

## API Documentation

  * AudioCapability
    * sample_rates
    * bit_depths
    * channels
    * available
  * AudioInfo
    * playback
    * recording
    * error
  * AudioConfig
    * sample_rate
    * bit_depth
    * channels
  * SoundServiceClient
    * SoundServiceClient
    * stub
    * get_audio_info
    * play_audio
    * record_audio
    * stop_recording

[ built with pdoc![pdoc
logo](data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20role%3D%22img%22%20aria-
label%3D%22pdoc%20logo%22%20width%3D%22300%22%20height%3D%22150%22%20viewBox%3D%22-1%200%2060%2030%22%3E%3Ctitle%3Epdoc%3C/title%3E%3Cpath%20d%3D%22M29.621%2021.293c-.011-.273-.214-.475-.511-.481a.5.5%200%200%200-.489.503l-.044%201.393c-.097.551-.695%201.215-1.566%201.704-.577.428-1.306.486-2.193.182-1.426-.617-2.467-1.654-3.304-2.487l-.173-.172a3.43%203.43%200%200%200-.365-.306.49.49%200%200%200-.286-.196c-1.718-1.06-4.931-1.47-7.353.191l-.219.15c-1.707%201.187-3.413%202.131-4.328%201.03-.02-.027-.49-.685-.141-1.763.233-.721.546-2.408.772-4.076.042-.09.067-.187.046-.288.166-1.347.277-2.625.241-3.351%201.378-1.008%202.271-2.586%202.271-4.362%200-.976-.272-1.935-.788-2.774-.057-.094-.122-.18-.184-.268.033-.167.052-.339.052-.516%200-1.477-1.202-2.679-2.679-2.679-.791%200-1.496.352-1.987.9a6.3%206.3%200%200%200-1.001.029c-.492-.564-1.207-.929-2.012-.929-1.477%200-2.679%201.202-2.679%202.679A2.65%202.65%200%200%200%20.97%206.554c-.383.747-.595%201.572-.595%202.41%200%202.311%201.507%204.29%203.635%205.107-.037.699-.147%202.27-.423%203.294l-.137.461c-.622%202.042-2.515%208.257%201.727%2010.643%201.614.908%203.06%201.248%204.317%201.248%202.665%200%204.492-1.524%205.322-2.401%201.476-1.559%202.886-1.854%206.491.82%201.877%201.393%203.514%201.753%204.861%201.068%202.223-1.713%202.811-3.867%203.399-6.374.077-.846.056-1.469.054-1.537zm-4.835%204.313c-.054.305-.156.586-.242.629-.034-.007-.131-.022-.307-.157-.145-.111-.314-.478-.456-.908.221.121.432.25.675.355.115.039.219.051.33.081zm-2.251-1.238c-.05.33-.158.648-.252.694-.022.001-.125-.018-.307-.157-.217-.166-.488-.906-.639-1.573.358.344.754.693%201.198%201.036zm-3.887-2.337c-.006-.116-.018-.231-.041-.342.635.145%201.189.368%201.599.625.097.231.166.481.174.642-.03.049-.055.101-.067.158-.046.013-.128.026-.298.004-.278-.037-.901-.57-1.367-1.087zm-1.127-.497c.116.306.176.625.12.71-.019.014-.117.045-.345.016-.206-.027-.604-.332-.986-.695.41-.051.816-.056%201.211-.031zm-4.535%201.535c.209.22.379.47.358.598-.006.041-.088.138-.351.234-.144.055-.539-.063-.979-.259a11.66%2011.66%200%200%200%20.972-.573zm.983-.664c.359-.237.738-.418%201.126-.554.25.237.479.548.457.694-.006.042-.087.138-.351.235-.174.064-.694-.105-1.232-.375zm-3.381%201.794c-.022.145-.061.29-.149.401-.133.166-.358.248-.69.251h-.002c-.133%200-.306-.26-.45-.621.417.091.854.07%201.291-.031zm-2.066-8.077a4.78%204.78%200%200%201-.775-.584c.172-.115.505-.254.88-.378l-.105.962zm-.331%202.302a10.32%2010.32%200%200%201-.828-.502c.202-.143.576-.328.984-.49l-.156.992zm-.45%202.157l-.701-.403c.214-.115.536-.249.891-.376a11.57%2011.57%200%200%201-.19.779zm-.181%201.716c.064.398.194.702.298.893-.194-.051-.435-.162-.736-.398.061-.119.224-.3.438-.495zM8.87%204.141c0%20.152-.123.276-.276.276s-.275-.124-.275-.276.123-.276.276-.276.275.124.275.276zm-.735-.389a1.15%201.15%200%200%200-.314.783%201.16%201.16%200%200%200%201.162%201.162c.457%200%20.842-.27%201.032-.653.026.117.042.238.042.362a1.68%201.68%200%200%201-1.679%201.679%201.68%201.68%200%200%201-1.679-1.679c0-.843.626-1.535%201.436-1.654zM5.059%205.406A1.68%201.68%200%200%201%203.38%207.085a1.68%201.68%200%200%201-1.679-1.679c0-.037.009-.072.011-.109.21.3.541.508.935.508a1.16%201.16%200%200%200%201.162-1.162%201.14%201.14%200%200%200-.474-.912c.015%200%20.03-.005.045-.005.926.001%201.679.754%201.679%201.68zM3.198%204.141c0%20.152-.123.276-.276.276s-.275-.124-.275-.276.123-.276.276-.276.275.124.275.276zM1.375%208.964c0-.52.103-1.035.288-1.52.466.394%201.06.64%201.717.64%201.144%200%202.116-.725%202.499-1.738.383%201.012%201.355%201.738%202.499%201.738.867%200%201.631-.421%202.121-1.062.307.605.478%201.267.478%201.942%200%202.486-2.153%204.51-4.801%204.51s-4.801-2.023-4.801-4.51zm24.342%2019.349c-.985.498-2.267.168-3.813-.979-3.073-2.281-5.453-3.199-7.813-.705-1.315%201.391-4.163%203.365-8.423.97-3.174-1.786-2.239-6.266-1.261-9.479l.146-.492c.276-1.02.395-2.457.444-3.268a6.11%206.11%200%200%200%201.18.115%206.01%206.01%200%200%200%202.536-.562l-.006.175c-.802.215-1.848.612-2.021%201.25-.079.295.021.601.274.837.219.203.415.364.598.501-.667.304-1.243.698-1.311%201.179-.02.144-.022.507.393.787.213.144.395.26.564.365-1.285.521-1.361.96-1.381%201.126-.018.142-.011.496.427.746l.854.489c-.473.389-.971.914-.999%201.429-.018.278.095.532.316.713.675.556%201.231.721%201.653.721.059%200%20.104-.014.158-.02.207.707.641%201.64%201.513%201.64h.013c.8-.008%201.236-.345%201.462-.626.173-.216.268-.457.325-.692.424.195.93.374%201.372.374.151%200%20.294-.021.423-.068.732-.27.944-.704.993-1.021.009-.061.003-.119.002-.179.266.086.538.147.789.147.15%200%20.294-.021.423-.069.542-.2.797-.489.914-.754.237.147.478.258.704.288.106.014.205.021.296.021.356%200%20.595-.101.767-.229.438.435%201.094.992%201.656%201.067.106.014.205.021.296.021a1.56%201.56%200%200%200%20.323-.035c.17.575.453%201.289.866%201.605.358.273.665.362.914.362a.99.99%200%200%200%20.421-.093%201.03%201.03%200%200%200%20.245-.164c.168.428.39.846.68%201.068.358.273.665.362.913.362a.99.99%200%200%200%20.421-.093c.317-.148.512-.448.639-.762.251.157.495.257.726.257.127%200%20.25-.024.37-.071.427-.17.706-.617.841-1.314.022-.015.047-.022.068-.038.067-.051.133-.104.196-.159-.443%201.486-1.107%202.761-2.086%203.257zM8.66%209.925a.5.5%200%201%200-1%200c0%20.653-.818%201.205-1.787%201.205s-1.787-.552-1.787-1.205a.5.5%200%201%200-1%200c0%201.216%201.25%202.205%202.787%202.205s2.787-.989%202.787-2.205zm4.4%2015.965l-.208.097c-2.661%201.258-4.708%201.436-6.086.527-1.542-1.017-1.88-3.19-1.844-4.198a.4.4%200%200%200-.385-.414c-.242-.029-.406.164-.414.385-.046%201.249.367%203.686%202.202%204.896.708.467%201.547.7%202.51.7%201.248%200%202.706-.392%204.362-1.174l.185-.086a.4.4%200%200%200%20.205-.527c-.089-.204-.326-.291-.527-.206zM9.547%202.292c.093.077.205.114.317.114a.5.5%200%200%200%20.318-.886L8.817.397a.5.5%200%200%200-.703.068.5.5%200%200%200%20.069.703l1.364%201.124zm-7.661-.065c.086%200%20.173-.022.253-.068l1.523-.893a.5.5%200%200%200-.506-.863l-1.523.892a.5.5%200%200%200-.179.685c.094.158.261.247.432.247z%22%20transform%3D%22matrix%28-1%200%200%201%2058%200%29%22%20fill%3D%22%233bb300%22/%3E%3Cpath%20d%3D%22M.3%2021.86V10.18q0-.46.02-.68.04-.22.18-.5.28-.54%201.34-.54%201.06%200%201.42.28.38.26.44.78.76-1.04%202.38-1.04%201.64%200%203.1%201.54%201.46%201.54%201.46%203.58%200%202.04-1.46%203.58-1.44%201.54-3.08%201.54-1.64%200-2.38-.92v4.04q0%20.46-.04.68-.02.22-.18.5-.14.3-.5.42-.36.12-.98.12-.62%200-1-.12-.36-.12-.52-.4-.14-.28-.18-.5-.02-.22-.02-.68zm3.96-9.42q-.46.54-.46%201.18%200%20.64.46%201.18.48.52%201.2.52.74%200%201.24-.52.52-.52.52-1.18%200-.66-.48-1.18-.48-.54-1.26-.54-.76%200-1.22.54zm14.741-8.36q.16-.3.54-.42.38-.12%201-.12.64%200%201.02.12.38.12.52.42.16.3.18.54.04.22.04.68v11.94q0%20.46-.04.7-.02.22-.18.5-.3.54-1.7.54-1.38%200-1.54-.98-.84.96-2.34.96-1.8%200-3.28-1.56-1.48-1.58-1.48-3.66%200-2.1%201.48-3.68%201.5-1.58%203.28-1.58%201.48%200%202.3%201v-4.2q0-.46.02-.68.04-.24.18-.52zm-3.24%2010.86q.52.54%201.26.54.74%200%201.22-.54.5-.54.5-1.18%200-.66-.48-1.22-.46-.56-1.26-.56-.8%200-1.28.56-.48.54-.48%201.2%200%20.66.52%201.2zm7.833-1.2q0-2.4%201.68-3.96%201.68-1.56%203.84-1.56%202.16%200%203.82%201.56%201.66%201.54%201.66%203.94%200%201.66-.86%202.96-.86%201.28-2.1%201.9-1.22.6-2.54.6-1.32%200-2.56-.64-1.24-.66-2.1-1.92-.84-1.28-.84-2.88zm4.18%201.44q.64.48%201.3.48.66%200%201.32-.5.66-.5.66-1.48%200-.98-.62-1.46-.62-.48-1.34-.48-.72%200-1.34.5-.62.5-.62%201.48%200%20.96.64%201.46zm11.412-1.44q0%20.84.56%201.32.56.46%201.18.46.64%200%201.18-.36.56-.38.9-.38.6%200%201.46%201.06.46.58.46%201.04%200%20.76-1.1%201.42-1.14.8-2.8.8-1.86%200-3.58-1.34-.82-.64-1.34-1.7-.52-1.08-.52-2.36%200-1.3.52-2.34.52-1.06%201.34-1.7%201.66-1.32%203.54-1.32.76%200%201.48.22.72.2%201.06.4l.32.2q.36.24.56.38.52.4.52.92%200%20.5-.42%201.14-.72%201.1-1.38%201.1-.38%200-1.08-.44-.36-.34-1.04-.34-.66%200-1.24.48-.58.48-.58%201.34z%22%20fill%3D%22green%22/%3E%3C/svg%3E)
](https://pdoc.dev "pdoc: Python API documentation generator")

#  [pykos](./../../pykos.html).[services](./../services.html).sound

Sound service client.

View Source

    
    
      1"""Sound service client."""
      2
      3from typing import AsyncGenerator, AsyncIterator, NotRequired, TypedDict, Unpack
      4
      5import grpc
      6import grpc.aio
      7from google.protobuf.empty_pb2 import Empty
      8
      9from kos_protos import common_pb2, sound_pb2, sound_pb2_grpc
     10
     11
     12class AudioCapability(TypedDict):
     13    """Information about audio capabilities.
     14
     15    Args:
     16        sample_rates: List of supported sample rates (e.g., 44100, 48000)
     17        bit_depths: List of supported bit depths (e.g., 16, 24, 32)
     18        channels: List of supported channel counts (e.g., 1, 2)
     19        available: Whether this capability is available
     20    """
     21
     22    sample_rates: list[int]
     23    bit_depths: list[int]
     24    channels: list[int]
     25    available: bool
     26
     27
     28class AudioInfo(TypedDict):
     29    """Information about audio system capabilities.
     30
     31    Args:
     32        playback: Playback capabilities
     33        recording: Recording capabilities
     34        error: Optional error information
     35    """
     36
     37    playback: AudioCapability
     38    recording: AudioCapability
     39    error: NotRequired[common_pb2.Error | None]
     40
     41
     42class AudioConfig(TypedDict):
     43    """Audio configuration parameters.
     44
     45    Args:
     46        sample_rate: Sample rate in Hz (e.g., 44100)
     47        bit_depth: Bit depth (e.g., 16)
     48        channels: Number of channels (1 for mono, 2 for stereo)
     49    """
     50
     51    sample_rate: int
     52    bit_depth: int
     53    channels: int
     54
     55
     56class SoundServiceClient:
     57    """Client for the SoundService.
     58
     59    This service allows playing audio through speakers and recording from microphones.
     60    """
     61
     62    def __init__(self, channel: grpc.aio.Channel) -> None:
     63        """Initialize the sound service client.
     64
     65        Args:
     66            channel: gRPC channel to use for communication.
     67        """
     68        self.stub = sound_pb2_grpc.SoundServiceStub(channel)
     69
     70    async def get_audio_info(self) -> AudioInfo:
     71        """Get information about audio capabilities.
     72
     73        Returns:
     74            AudioInfo containing playback and recording capabilities.
     75        """
     76        return await self.stub.GetAudioInfo(Empty())
     77
     78    async def play_audio(
     79        self, audio_iterator: AsyncIterator[bytes], **kwargs: Unpack[AudioConfig]
     80    ) -> common_pb2.ActionResponse:
     81        """Stream PCM audio data to the speaker.
     82
     83        Args:
     84            audio_iterator: Iterator yielding chunks of PCM audio data
     85            **kwargs: Audio configuration parameters
     86                sample_rate: Sample rate in Hz (e.g., 44100)
     87                bit_depth: Bit depth (e.g., 16)
     88                channels: Number of channels (1 for mono, 2 for stereo)
     89
     90        Returns:
     91            ActionResponse indicating success/failure of the playback operation.
     92
     93        Example:
     94            >>> config = AudioConfig(sample_rate=44100, bit_depth=16, channels=2)
     95            >>> with open('audio.raw', 'rb') as f:
     96            ...     def chunks():
     97            ...         while chunk := f.read(4096):
     98            ...             yield chunk
     99            ...     response = client.play_audio(chunks(), config)
    100        """
    101
    102        async def request_iterator() -> AsyncGenerator[sound_pb2.PlayAudioRequest, None]:
    103            # First message includes config
    104            yield sound_pb2.PlayAudioRequest(
    105                config=sound_pb2.AudioConfig(**kwargs),
    106            )
    107            # Subsequent messages contain audio data
    108            async for chunk in audio_iterator:
    109                yield sound_pb2.PlayAudioRequest(audio_data=chunk)
    110
    111        return await self.stub.PlayAudio(request_iterator())
    112
    113    async def record_audio(self, duration_ms: int = 0, **kwargs: Unpack[AudioConfig]) -> AsyncGenerator[bytes, None]:
    114        """Record PCM audio data from the microphone.
    115
    116        Args:
    117            duration_ms: Recording duration in milliseconds (0 for continuous)
    118            **kwargs: Audio configuration parameters
    119                sample_rate: Sample rate in Hz (e.g., 44100)
    120                bit_depth: Bit depth (e.g., 16)
    121                channels: Number of channels (1 for mono, 2 for stereo)
    122
    123        Yields:
    124            Chunks of PCM audio data.
    125
    126        Example:
    127            >>> config = AudioConfig(sample_rate=44100, bit_depth=16, channels=1)
    128            >>> with open('recording.raw', 'wb') as f:
    129            ...     for chunk in client.record_audio(duration_ms=5000, **config):
    130            ...         f.write(chunk)
    131        """
    132        request = sound_pb2.RecordAudioRequest(
    133            config=sound_pb2.AudioConfig(**kwargs),
    134            duration_ms=duration_ms,
    135        )
    136
    137        async for response in self.stub.RecordAudio(request):
    138            if response.HasField("error"):
    139                raise RuntimeError(f"Recording error: {response.error}")
    140            yield response.audio_data
    141
    142    async def stop_recording(self) -> common_pb2.ActionResponse:
    143        """Stop an ongoing recording session.
    144
    145        Returns:
    146            ActionResponse indicating success/failure of the stop operation.
    147        """
    148        return await self.stub.StopRecording(Empty())
    

class AudioCapability(typing.TypedDict): View Source

    
    
    13class AudioCapability(TypedDict):
    14    """Information about audio capabilities.
    15
    16    Args:
    17        sample_rates: List of supported sample rates (e.g., 44100, 48000)
    18        bit_depths: List of supported bit depths (e.g., 16, 24, 32)
    19        channels: List of supported channel counts (e.g., 1, 2)
    20        available: Whether this capability is available
    21    """
    22
    23    sample_rates: list[int]
    24    bit_depths: list[int]
    25    channels: list[int]
    26    available: bool
    

Information about audio capabilities.

Args: sample_rates: List of supported sample rates (e.g., 44100, 48000)
bit_depths: List of supported bit depths (e.g., 16, 24, 32) channels: List of
supported channel counts (e.g., 1, 2) available: Whether this capability is
available

sample_rates: list[int]

bit_depths: list[int]

channels: list[int]

available: bool

class AudioInfo(typing.TypedDict): View Source

    
    
    29class AudioInfo(TypedDict):
    30    """Information about audio system capabilities.
    31
    32    Args:
    33        playback: Playback capabilities
    34        recording: Recording capabilities
    35        error: Optional error information
    36    """
    37
    38    playback: AudioCapability
    39    recording: AudioCapability
    40    error: NotRequired[common_pb2.Error | None]
    

Information about audio system capabilities.

Args: playback: Playback capabilities recording: Recording capabilities error:
Optional error information

playback: AudioCapability

recording: AudioCapability

error: NotRequired[kos.common_pb2.Error | None]

class AudioConfig(typing.TypedDict): View Source

    
    
    43class AudioConfig(TypedDict):
    44    """Audio configuration parameters.
    45
    46    Args:
    47        sample_rate: Sample rate in Hz (e.g., 44100)
    48        bit_depth: Bit depth (e.g., 16)
    49        channels: Number of channels (1 for mono, 2 for stereo)
    50    """
    51
    52    sample_rate: int
    53    bit_depth: int
    54    channels: int
    

Audio configuration parameters.

Args: sample_rate: Sample rate in Hz (e.g., 44100) bit_depth: Bit depth (e.g.,
16) channels: Number of channels (1 for mono, 2 for stereo)

sample_rate: int

bit_depth: int

channels: int

class SoundServiceClient: View Source

    
    
     57class SoundServiceClient:
     58    """Client for the SoundService.
     59
     60    This service allows playing audio through speakers and recording from microphones.
     61    """
     62
     63    def __init__(self, channel: grpc.aio.Channel) -> None:
     64        """Initialize the sound service client.
     65
     66        Args:
     67            channel: gRPC channel to use for communication.
     68        """
     69        self.stub = sound_pb2_grpc.SoundServiceStub(channel)
     70
     71    async def get_audio_info(self) -> AudioInfo:
     72        """Get information about audio capabilities.
     73
     74        Returns:
     75            AudioInfo containing playback and recording capabilities.
     76        """
     77        return await self.stub.GetAudioInfo(Empty())
     78
     79    async def play_audio(
     80        self, audio_iterator: AsyncIterator[bytes], **kwargs: Unpack[AudioConfig]
     81    ) -> common_pb2.ActionResponse:
     82        """Stream PCM audio data to the speaker.
     83
     84        Args:
     85            audio_iterator: Iterator yielding chunks of PCM audio data
     86            **kwargs: Audio configuration parameters
     87                sample_rate: Sample rate in Hz (e.g., 44100)
     88                bit_depth: Bit depth (e.g., 16)
     89                channels: Number of channels (1 for mono, 2 for stereo)
     90
     91        Returns:
     92            ActionResponse indicating success/failure of the playback operation.
     93
     94        Example:
     95            >>> config = AudioConfig(sample_rate=44100, bit_depth=16, channels=2)
     96            >>> with open('audio.raw', 'rb') as f:
     97            ...     def chunks():
     98            ...         while chunk := f.read(4096):
     99            ...             yield chunk
    100            ...     response = client.play_audio(chunks(), config)
    101        """
    102
    103        async def request_iterator() -> AsyncGenerator[sound_pb2.PlayAudioRequest, None]:
    104            # First message includes config
    105            yield sound_pb2.PlayAudioRequest(
    106                config=sound_pb2.AudioConfig(**kwargs),
    107            )
    108            # Subsequent messages contain audio data
    109            async for chunk in audio_iterator:
    110                yield sound_pb2.PlayAudioRequest(audio_data=chunk)
    111
    112        return await self.stub.PlayAudio(request_iterator())
    113
    114    async def record_audio(self, duration_ms: int = 0, **kwargs: Unpack[AudioConfig]) -> AsyncGenerator[bytes, None]:
    115        """Record PCM audio data from the microphone.
    116
    117        Args:
    118            duration_ms: Recording duration in milliseconds (0 for continuous)
    119            **kwargs: Audio configuration parameters
    120                sample_rate: Sample rate in Hz (e.g., 44100)
    121                bit_depth: Bit depth (e.g., 16)
    122                channels: Number of channels (1 for mono, 2 for stereo)
    123
    124        Yields:
    125            Chunks of PCM audio data.
    126
    127        Example:
    128            >>> config = AudioConfig(sample_rate=44100, bit_depth=16, channels=1)
    129            >>> with open('recording.raw', 'wb') as f:
    130            ...     for chunk in client.record_audio(duration_ms=5000, **config):
    131            ...         f.write(chunk)
    132        """
    133        request = sound_pb2.RecordAudioRequest(
    134            config=sound_pb2.AudioConfig(**kwargs),
    135            duration_ms=duration_ms,
    136        )
    137
    138        async for response in self.stub.RecordAudio(request):
    139            if response.HasField("error"):
    140                raise RuntimeError(f"Recording error: {response.error}")
    141            yield response.audio_data
    142
    143    async def stop_recording(self) -> common_pb2.ActionResponse:
    144        """Stop an ongoing recording session.
    145
    146        Returns:
    147            ActionResponse indicating success/failure of the stop operation.
    148        """
    149        return await self.stub.StopRecording(Empty())
    

Client for the SoundService.

This service allows playing audio through speakers and recording from
microphones.

SoundServiceClient(channel: grpc.aio._base_channel.Channel) View Source

    
    
    63    def __init__(self, channel: grpc.aio.Channel) -> None:
    64        """Initialize the sound service client.
    65
    66        Args:
    67            channel: gRPC channel to use for communication.
    68        """
    69        self.stub = sound_pb2_grpc.SoundServiceStub(channel)
    

Initialize the sound service client.

Args: channel: gRPC channel to use for communication.

stub

async def get_audio_info(self) -> AudioInfo: View Source

    
    
    71    async def get_audio_info(self) -> AudioInfo:
    72        """Get information about audio capabilities.
    73
    74        Returns:
    75            AudioInfo containing playback and recording capabilities.
    76        """
    77        return await self.stub.GetAudioInfo(Empty())
    

Get information about audio capabilities.

Returns: AudioInfo containing playback and recording capabilities.

async def play_audio( self, audio_iterator: AsyncIterator[bytes], **kwargs:
*<class 'AudioConfig'>) -> kos.common_pb2.ActionResponse: View Source

    
    
     79    async def play_audio(
     80        self, audio_iterator: AsyncIterator[bytes], **kwargs: Unpack[AudioConfig]
     81    ) -> common_pb2.ActionResponse:
     82        """Stream PCM audio data to the speaker.
     83
     84        Args:
     85            audio_iterator: Iterator yielding chunks of PCM audio data
     86            **kwargs: Audio configuration parameters
     87                sample_rate: Sample rate in Hz (e.g., 44100)
     88                bit_depth: Bit depth (e.g., 16)
     89                channels: Number of channels (1 for mono, 2 for stereo)
     90
     91        Returns:
     92            ActionResponse indicating success/failure of the playback operation.
     93
     94        Example:
     95            >>> config = AudioConfig(sample_rate=44100, bit_depth=16, channels=2)
     96            >>> with open('audio.raw', 'rb') as f:
     97            ...     def chunks():
     98            ...         while chunk := f.read(4096):
     99            ...             yield chunk
    100            ...     response = client.play_audio(chunks(), config)
    101        """
    102
    103        async def request_iterator() -> AsyncGenerator[sound_pb2.PlayAudioRequest, None]:
    104            # First message includes config
    105            yield sound_pb2.PlayAudioRequest(
    106                config=sound_pb2.AudioConfig(**kwargs),
    107            )
    108            # Subsequent messages contain audio data
    109            async for chunk in audio_iterator:
    110                yield sound_pb2.PlayAudioRequest(audio_data=chunk)
    111
    112        return await self.stub.PlayAudio(request_iterator())
    

Stream PCM audio data to the speaker.

Args: audio_iterator: Iterator yielding chunks of PCM audio data **kwargs:
Audio configuration parameters sample_rate: Sample rate in Hz (e.g., 44100)
bit_depth: Bit depth (e.g., 16) channels: Number of channels (1 for mono, 2
for stereo)

Returns: ActionResponse indicating success/failure of the playback operation.

Example:

> > > config = AudioConfig(sample_rate=44100, bit_depth=16, channels=2) with
> open('audio.raw', 'rb') as f: ... def chunks(): ... while chunk :=
> f.read(4096): ... yield chunk ... response = client.play_audio(chunks(),
> config)

async def record_audio( self, duration_ms: int = 0, **kwargs: *<class
'AudioConfig'>) -> AsyncGenerator[bytes, NoneType]: View Source

    
    
    114    async def record_audio(self, duration_ms: int = 0, **kwargs: Unpack[AudioConfig]) -> AsyncGenerator[bytes, None]:
    115        """Record PCM audio data from the microphone.
    116
    117        Args:
    118            duration_ms: Recording duration in milliseconds (0 for continuous)
    119            **kwargs: Audio configuration parameters
    120                sample_rate: Sample rate in Hz (e.g., 44100)
    121                bit_depth: Bit depth (e.g., 16)
    122                channels: Number of channels (1 for mono, 2 for stereo)
    123
    124        Yields:
    125            Chunks of PCM audio data.
    126
    127        Example:
    128            >>> config = AudioConfig(sample_rate=44100, bit_depth=16, channels=1)
    129            >>> with open('recording.raw', 'wb') as f:
    130            ...     for chunk in client.record_audio(duration_ms=5000, **config):
    131            ...         f.write(chunk)
    132        """
    133        request = sound_pb2.RecordAudioRequest(
    134            config=sound_pb2.AudioConfig(**kwargs),
    135            duration_ms=duration_ms,
    136        )
    137
    138        async for response in self.stub.RecordAudio(request):
    139            if response.HasField("error"):
    140                raise RuntimeError(f"Recording error: {response.error}")
    141            yield response.audio_data
    

Record PCM audio data from the microphone.

Args: duration_ms: Recording duration in milliseconds (0 for continuous)
**kwargs: Audio configuration parameters sample_rate: Sample rate in Hz (e.g.,
44100) bit_depth: Bit depth (e.g., 16) channels: Number of channels (1 for
mono, 2 for stereo)

Yields: Chunks of PCM audio data.

Example:

> > > config = AudioConfig(sample_rate=44100, bit_depth=16, channels=1) with
> open('recording.raw', 'wb') as f: ... for chunk in
> client.record_audio(duration_ms=5000, **config): ... f.write(chunk)

async def stop_recording(self) -> kos.common_pb2.ActionResponse: View Source

    
    
    143    async def stop_recording(self) -> common_pb2.ActionResponse:
    144        """Stop an ongoing recording session.
    145
    146        Returns:
    147            ActionResponse indicating success/failure of the stop operation.
    148        """
    149        return await self.stub.StopRecording(Empty())
    

Stop an ongoing recording session.

Returns: ActionResponse indicating success/failure of the stop operation.

