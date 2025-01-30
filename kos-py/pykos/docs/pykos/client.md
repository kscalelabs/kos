[  pykos](../pykos.html)

## API Documentation

  * KOS
    * KOS
    * ip
    * port
    * imu
    * actuator
    * led_matrix
    * sound
    * process_manager
    * inference
    * sim
    * connect
    * close

[ built with pdoc![pdoc
logo](data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20role%3D%22img%22%20aria-
label%3D%22pdoc%20logo%22%20width%3D%22300%22%20height%3D%22150%22%20viewBox%3D%22-1%200%2060%2030%22%3E%3Ctitle%3Epdoc%3C/title%3E%3Cpath%20d%3D%22M29.621%2021.293c-.011-.273-.214-.475-.511-.481a.5.5%200%200%200-.489.503l-.044%201.393c-.097.551-.695%201.215-1.566%201.704-.577.428-1.306.486-2.193.182-1.426-.617-2.467-1.654-3.304-2.487l-.173-.172a3.43%203.43%200%200%200-.365-.306.49.49%200%200%200-.286-.196c-1.718-1.06-4.931-1.47-7.353.191l-.219.15c-1.707%201.187-3.413%202.131-4.328%201.03-.02-.027-.49-.685-.141-1.763.233-.721.546-2.408.772-4.076.042-.09.067-.187.046-.288.166-1.347.277-2.625.241-3.351%201.378-1.008%202.271-2.586%202.271-4.362%200-.976-.272-1.935-.788-2.774-.057-.094-.122-.18-.184-.268.033-.167.052-.339.052-.516%200-1.477-1.202-2.679-2.679-2.679-.791%200-1.496.352-1.987.9a6.3%206.3%200%200%200-1.001.029c-.492-.564-1.207-.929-2.012-.929-1.477%200-2.679%201.202-2.679%202.679A2.65%202.65%200%200%200%20.97%206.554c-.383.747-.595%201.572-.595%202.41%200%202.311%201.507%204.29%203.635%205.107-.037.699-.147%202.27-.423%203.294l-.137.461c-.622%202.042-2.515%208.257%201.727%2010.643%201.614.908%203.06%201.248%204.317%201.248%202.665%200%204.492-1.524%205.322-2.401%201.476-1.559%202.886-1.854%206.491.82%201.877%201.393%203.514%201.753%204.861%201.068%202.223-1.713%202.811-3.867%203.399-6.374.077-.846.056-1.469.054-1.537zm-4.835%204.313c-.054.305-.156.586-.242.629-.034-.007-.131-.022-.307-.157-.145-.111-.314-.478-.456-.908.221.121.432.25.675.355.115.039.219.051.33.081zm-2.251-1.238c-.05.33-.158.648-.252.694-.022.001-.125-.018-.307-.157-.217-.166-.488-.906-.639-1.573.358.344.754.693%201.198%201.036zm-3.887-2.337c-.006-.116-.018-.231-.041-.342.635.145%201.189.368%201.599.625.097.231.166.481.174.642-.03.049-.055.101-.067.158-.046.013-.128.026-.298.004-.278-.037-.901-.57-1.367-1.087zm-1.127-.497c.116.306.176.625.12.71-.019.014-.117.045-.345.016-.206-.027-.604-.332-.986-.695.41-.051.816-.056%201.211-.031zm-4.535%201.535c.209.22.379.47.358.598-.006.041-.088.138-.351.234-.144.055-.539-.063-.979-.259a11.66%2011.66%200%200%200%20.972-.573zm.983-.664c.359-.237.738-.418%201.126-.554.25.237.479.548.457.694-.006.042-.087.138-.351.235-.174.064-.694-.105-1.232-.375zm-3.381%201.794c-.022.145-.061.29-.149.401-.133.166-.358.248-.69.251h-.002c-.133%200-.306-.26-.45-.621.417.091.854.07%201.291-.031zm-2.066-8.077a4.78%204.78%200%200%201-.775-.584c.172-.115.505-.254.88-.378l-.105.962zm-.331%202.302a10.32%2010.32%200%200%201-.828-.502c.202-.143.576-.328.984-.49l-.156.992zm-.45%202.157l-.701-.403c.214-.115.536-.249.891-.376a11.57%2011.57%200%200%201-.19.779zm-.181%201.716c.064.398.194.702.298.893-.194-.051-.435-.162-.736-.398.061-.119.224-.3.438-.495zM8.87%204.141c0%20.152-.123.276-.276.276s-.275-.124-.275-.276.123-.276.276-.276.275.124.275.276zm-.735-.389a1.15%201.15%200%200%200-.314.783%201.16%201.16%200%200%200%201.162%201.162c.457%200%20.842-.27%201.032-.653.026.117.042.238.042.362a1.68%201.68%200%200%201-1.679%201.679%201.68%201.68%200%200%201-1.679-1.679c0-.843.626-1.535%201.436-1.654zM5.059%205.406A1.68%201.68%200%200%201%203.38%207.085a1.68%201.68%200%200%201-1.679-1.679c0-.037.009-.072.011-.109.21.3.541.508.935.508a1.16%201.16%200%200%200%201.162-1.162%201.14%201.14%200%200%200-.474-.912c.015%200%20.03-.005.045-.005.926.001%201.679.754%201.679%201.68zM3.198%204.141c0%20.152-.123.276-.276.276s-.275-.124-.275-.276.123-.276.276-.276.275.124.275.276zM1.375%208.964c0-.52.103-1.035.288-1.52.466.394%201.06.64%201.717.64%201.144%200%202.116-.725%202.499-1.738.383%201.012%201.355%201.738%202.499%201.738.867%200%201.631-.421%202.121-1.062.307.605.478%201.267.478%201.942%200%202.486-2.153%204.51-4.801%204.51s-4.801-2.023-4.801-4.51zm24.342%2019.349c-.985.498-2.267.168-3.813-.979-3.073-2.281-5.453-3.199-7.813-.705-1.315%201.391-4.163%203.365-8.423.97-3.174-1.786-2.239-6.266-1.261-9.479l.146-.492c.276-1.02.395-2.457.444-3.268a6.11%206.11%200%200%200%201.18.115%206.01%206.01%200%200%200%202.536-.562l-.006.175c-.802.215-1.848.612-2.021%201.25-.079.295.021.601.274.837.219.203.415.364.598.501-.667.304-1.243.698-1.311%201.179-.02.144-.022.507.393.787.213.144.395.26.564.365-1.285.521-1.361.96-1.381%201.126-.018.142-.011.496.427.746l.854.489c-.473.389-.971.914-.999%201.429-.018.278.095.532.316.713.675.556%201.231.721%201.653.721.059%200%20.104-.014.158-.02.207.707.641%201.64%201.513%201.64h.013c.8-.008%201.236-.345%201.462-.626.173-.216.268-.457.325-.692.424.195.93.374%201.372.374.151%200%20.294-.021.423-.068.732-.27.944-.704.993-1.021.009-.061.003-.119.002-.179.266.086.538.147.789.147.15%200%20.294-.021.423-.069.542-.2.797-.489.914-.754.237.147.478.258.704.288.106.014.205.021.296.021.356%200%20.595-.101.767-.229.438.435%201.094.992%201.656%201.067.106.014.205.021.296.021a1.56%201.56%200%200%200%20.323-.035c.17.575.453%201.289.866%201.605.358.273.665.362.914.362a.99.99%200%200%200%20.421-.093%201.03%201.03%200%200%200%20.245-.164c.168.428.39.846.68%201.068.358.273.665.362.913.362a.99.99%200%200%200%20.421-.093c.317-.148.512-.448.639-.762.251.157.495.257.726.257.127%200%20.25-.024.37-.071.427-.17.706-.617.841-1.314.022-.015.047-.022.068-.038.067-.051.133-.104.196-.159-.443%201.486-1.107%202.761-2.086%203.257zM8.66%209.925a.5.5%200%201%200-1%200c0%20.653-.818%201.205-1.787%201.205s-1.787-.552-1.787-1.205a.5.5%200%201%200-1%200c0%201.216%201.25%202.205%202.787%202.205s2.787-.989%202.787-2.205zm4.4%2015.965l-.208.097c-2.661%201.258-4.708%201.436-6.086.527-1.542-1.017-1.88-3.19-1.844-4.198a.4.4%200%200%200-.385-.414c-.242-.029-.406.164-.414.385-.046%201.249.367%203.686%202.202%204.896.708.467%201.547.7%202.51.7%201.248%200%202.706-.392%204.362-1.174l.185-.086a.4.4%200%200%200%20.205-.527c-.089-.204-.326-.291-.527-.206zM9.547%202.292c.093.077.205.114.317.114a.5.5%200%200%200%20.318-.886L8.817.397a.5.5%200%200%200-.703.068.5.5%200%200%200%20.069.703l1.364%201.124zm-7.661-.065c.086%200%20.173-.022.253-.068l1.523-.893a.5.5%200%200%200-.506-.863l-1.523.892a.5.5%200%200%200-.179.685c.094.158.261.247.432.247z%22%20transform%3D%22matrix%28-1%200%200%201%2058%200%29%22%20fill%3D%22%233bb300%22/%3E%3Cpath%20d%3D%22M.3%2021.86V10.18q0-.46.02-.68.04-.22.18-.5.28-.54%201.34-.54%201.06%200%201.42.28.38.26.44.78.76-1.04%202.38-1.04%201.64%200%203.1%201.54%201.46%201.54%201.46%203.58%200%202.04-1.46%203.58-1.44%201.54-3.08%201.54-1.64%200-2.38-.92v4.04q0%20.46-.04.68-.02.22-.18.5-.14.3-.5.42-.36.12-.98.12-.62%200-1-.12-.36-.12-.52-.4-.14-.28-.18-.5-.02-.22-.02-.68zm3.96-9.42q-.46.54-.46%201.18%200%20.64.46%201.18.48.52%201.2.52.74%200%201.24-.52.52-.52.52-1.18%200-.66-.48-1.18-.48-.54-1.26-.54-.76%200-1.22.54zm14.741-8.36q.16-.3.54-.42.38-.12%201-.12.64%200%201.02.12.38.12.52.42.16.3.18.54.04.22.04.68v11.94q0%20.46-.04.7-.02.22-.18.5-.3.54-1.7.54-1.38%200-1.54-.98-.84.96-2.34.96-1.8%200-3.28-1.56-1.48-1.58-1.48-3.66%200-2.1%201.48-3.68%201.5-1.58%203.28-1.58%201.48%200%202.3%201v-4.2q0-.46.02-.68.04-.24.18-.52zm-3.24%2010.86q.52.54%201.26.54.74%200%201.22-.54.5-.54.5-1.18%200-.66-.48-1.22-.46-.56-1.26-.56-.8%200-1.28.56-.48.54-.48%201.2%200%20.66.52%201.2zm7.833-1.2q0-2.4%201.68-3.96%201.68-1.56%203.84-1.56%202.16%200%203.82%201.56%201.66%201.54%201.66%203.94%200%201.66-.86%202.96-.86%201.28-2.1%201.9-1.22.6-2.54.6-1.32%200-2.56-.64-1.24-.66-2.1-1.92-.84-1.28-.84-2.88zm4.18%201.44q.64.48%201.3.48.66%200%201.32-.5.66-.5.66-1.48%200-.98-.62-1.46-.62-.48-1.34-.48-.72%200-1.34.5-.62.5-.62%201.48%200%20.96.64%201.46zm11.412-1.44q0%20.84.56%201.32.56.46%201.18.46.64%200%201.18-.36.56-.38.9-.38.6%200%201.46%201.06.46.58.46%201.04%200%20.76-1.1%201.42-1.14.8-2.8.8-1.86%200-3.58-1.34-.82-.64-1.34-1.7-.52-1.08-.52-2.36%200-1.3.52-2.34.52-1.06%201.34-1.7%201.66-1.32%203.54-1.32.76%200%201.48.22.72.2%201.06.4l.32.2q.36.24.56.38.52.4.52.92%200%20.5-.42%201.14-.72%201.1-1.38%201.1-.38%200-1.08-.44-.36-.34-1.04-.34-.66%200-1.24.48-.58.48-.58%201.34z%22%20fill%3D%22green%22/%3E%3C/svg%3E)
](https://pdoc.dev "pdoc: Python API documentation generator")

#  [pykos](./../pykos.html).client

KOS client.

View Source

    
    
      1"""KOS client."""
      2
      3from typing import Any
      4
      5import grpc
      6import grpc.aio
      7
      8from pykos.services.actuator import ActuatorServiceClient
      9from pykos.services.imu import IMUServiceClient
     10from pykos.services.inference import InferenceServiceClient
     11from pykos.services.led_matrix import LEDMatrixServiceClient
     12from pykos.services.process_manager import ProcessManagerServiceClient
     13from pykos.services.sim import SimServiceClient
     14from pykos.services.sound import SoundServiceClient
     15
     16
     17class KOS:
     18    """KOS client.
     19
     20    Args:
     21        ip (str, optional): IP address of the robot running KOS. Defaults to localhost.
     22        port (int, optional): Port of the robot running KOS. Defaults to 50051.
     23
     24    Attributes:
     25        imu (IMUServiceClient): Client for the IMU service.
     26    """
     27
     28    def __init__(self, ip: str = "localhost", port: int = 50051) -> None:
     29        self.ip = ip
     30        self.port = port
     31        self._channel: grpc.aio.Channel | None = None
     32        self._imu: IMUServiceClient | None = None
     33        self._actuator: ActuatorServiceClient | None = None
     34        self._led_matrix: LEDMatrixServiceClient | None = None
     35        self._sound: SoundServiceClient | None = None
     36        self._process_manager: ProcessManagerServiceClient | None = None
     37        self._inference: InferenceServiceClient | None = None
     38        self._sim: SimServiceClient | None = None
     39
     40    @property
     41    def imu(self) -> IMUServiceClient:
     42        if self._imu is None:
     43            raise RuntimeError("IMU client not initialized! Must call __aenter__() first.")
     44        return self._imu
     45
     46    @property
     47    def actuator(self) -> ActuatorServiceClient:
     48        if self._actuator is None:
     49            raise RuntimeError("Actuator client not initialized! Must call __aenter__() first.")
     50        return self._actuator
     51
     52    @property
     53    def led_matrix(self) -> LEDMatrixServiceClient:
     54        if self._led_matrix is None:
     55            raise RuntimeError("LED Matrix client not initialized! Must call __aenter__() first.")
     56        return self._led_matrix
     57
     58    @property
     59    def sound(self) -> SoundServiceClient:
     60        if self._sound is None:
     61            raise RuntimeError("Sound client not initialized! Must call __aenter__() first.")
     62        return self._sound
     63
     64    @property
     65    def process_manager(self) -> ProcessManagerServiceClient:
     66        if self._process_manager is None:
     67            raise RuntimeError("Process Manager client not initialized! Must call __aenter__() first.")
     68        return self._process_manager
     69
     70    @property
     71    def inference(self) -> InferenceServiceClient:
     72        if self._inference is None:
     73            raise RuntimeError("Inference client not initialized! Must call __aenter__() first.")
     74        return self._inference
     75
     76    @property
     77    def sim(self) -> SimServiceClient:
     78        if self._sim is None:
     79            raise RuntimeError("Sim client not initialized! Must call __aenter__() first.")
     80        return self._sim
     81
     82    async def connect(self) -> None:
     83        """Connect to the gRPC server and initialize service clients."""
     84        self._channel = grpc.aio.insecure_channel(f"{self.ip}:{self.port}")
     85        self._imu = IMUServiceClient(self._channel)
     86        self._actuator = ActuatorServiceClient(self._channel)
     87        self._led_matrix = LEDMatrixServiceClient(self._channel)
     88        self._sound = SoundServiceClient(self._channel)
     89        self._process_manager = ProcessManagerServiceClient(self._channel)
     90        self._inference = InferenceServiceClient(self._channel)
     91        self._sim = SimServiceClient(self._channel)
     92
     93    async def close(self) -> None:
     94        """Close the gRPC channel."""
     95        if self._channel is not None:
     96            await self._channel.close()
     97
     98    async def __aenter__(self) -> "KOS":
     99        await self.connect()
    100        return self
    101
    102    async def __aexit__(self, exc_type: Any, exc_value: Any, traceback: Any) -> None:  # noqa: ANN401
    103        await self.close()
    

class KOS: View Source

    
    
     18class KOS:
     19    """KOS client.
     20
     21    Args:
     22        ip (str, optional): IP address of the robot running KOS. Defaults to localhost.
     23        port (int, optional): Port of the robot running KOS. Defaults to 50051.
     24
     25    Attributes:
     26        imu (IMUServiceClient): Client for the IMU service.
     27    """
     28
     29    def __init__(self, ip: str = "localhost", port: int = 50051) -> None:
     30        self.ip = ip
     31        self.port = port
     32        self._channel: grpc.aio.Channel | None = None
     33        self._imu: IMUServiceClient | None = None
     34        self._actuator: ActuatorServiceClient | None = None
     35        self._led_matrix: LEDMatrixServiceClient | None = None
     36        self._sound: SoundServiceClient | None = None
     37        self._process_manager: ProcessManagerServiceClient | None = None
     38        self._inference: InferenceServiceClient | None = None
     39        self._sim: SimServiceClient | None = None
     40
     41    @property
     42    def imu(self) -> IMUServiceClient:
     43        if self._imu is None:
     44            raise RuntimeError("IMU client not initialized! Must call __aenter__() first.")
     45        return self._imu
     46
     47    @property
     48    def actuator(self) -> ActuatorServiceClient:
     49        if self._actuator is None:
     50            raise RuntimeError("Actuator client not initialized! Must call __aenter__() first.")
     51        return self._actuator
     52
     53    @property
     54    def led_matrix(self) -> LEDMatrixServiceClient:
     55        if self._led_matrix is None:
     56            raise RuntimeError("LED Matrix client not initialized! Must call __aenter__() first.")
     57        return self._led_matrix
     58
     59    @property
     60    def sound(self) -> SoundServiceClient:
     61        if self._sound is None:
     62            raise RuntimeError("Sound client not initialized! Must call __aenter__() first.")
     63        return self._sound
     64
     65    @property
     66    def process_manager(self) -> ProcessManagerServiceClient:
     67        if self._process_manager is None:
     68            raise RuntimeError("Process Manager client not initialized! Must call __aenter__() first.")
     69        return self._process_manager
     70
     71    @property
     72    def inference(self) -> InferenceServiceClient:
     73        if self._inference is None:
     74            raise RuntimeError("Inference client not initialized! Must call __aenter__() first.")
     75        return self._inference
     76
     77    @property
     78    def sim(self) -> SimServiceClient:
     79        if self._sim is None:
     80            raise RuntimeError("Sim client not initialized! Must call __aenter__() first.")
     81        return self._sim
     82
     83    async def connect(self) -> None:
     84        """Connect to the gRPC server and initialize service clients."""
     85        self._channel = grpc.aio.insecure_channel(f"{self.ip}:{self.port}")
     86        self._imu = IMUServiceClient(self._channel)
     87        self._actuator = ActuatorServiceClient(self._channel)
     88        self._led_matrix = LEDMatrixServiceClient(self._channel)
     89        self._sound = SoundServiceClient(self._channel)
     90        self._process_manager = ProcessManagerServiceClient(self._channel)
     91        self._inference = InferenceServiceClient(self._channel)
     92        self._sim = SimServiceClient(self._channel)
     93
     94    async def close(self) -> None:
     95        """Close the gRPC channel."""
     96        if self._channel is not None:
     97            await self._channel.close()
     98
     99    async def __aenter__(self) -> "KOS":
    100        await self.connect()
    101        return self
    102
    103    async def __aexit__(self, exc_type: Any, exc_value: Any, traceback: Any) -> None:  # noqa: ANN401
    104        await self.close()
    

KOS client.

Args: ip (str, optional): IP address of the robot running KOS. Defaults to
localhost. port (int, optional): Port of the robot running KOS. Defaults to
50051.

Attributes: imu (IMUServiceClient): Client for the IMU service.

KOS(ip: str = 'localhost', port: int = 50051) View Source

    
    
    29    def __init__(self, ip: str = "localhost", port: int = 50051) -> None:
    30        self.ip = ip
    31        self.port = port
    32        self._channel: grpc.aio.Channel | None = None
    33        self._imu: IMUServiceClient | None = None
    34        self._actuator: ActuatorServiceClient | None = None
    35        self._led_matrix: LEDMatrixServiceClient | None = None
    36        self._sound: SoundServiceClient | None = None
    37        self._process_manager: ProcessManagerServiceClient | None = None
    38        self._inference: InferenceServiceClient | None = None
    39        self._sim: SimServiceClient | None = None
    

ip

port

imu: [pykos.services.imu.IMUServiceClient](services/imu.html#IMUServiceClient)
View Source

    
    
    41    @property
    42    def imu(self) -> IMUServiceClient:
    43        if self._imu is None:
    44            raise RuntimeError("IMU client not initialized! Must call __aenter__() first.")
    45        return self._imu
    

actuator:
[pykos.services.actuator.ActuatorServiceClient](services/actuator.html#ActuatorServiceClient)
View Source

    
    
    47    @property
    48    def actuator(self) -> ActuatorServiceClient:
    49        if self._actuator is None:
    50            raise RuntimeError("Actuator client not initialized! Must call __aenter__() first.")
    51        return self._actuator
    

led_matrix:
[pykos.services.led_matrix.LEDMatrixServiceClient](services/led_matrix.html#LEDMatrixServiceClient)
View Source

    
    
    53    @property
    54    def led_matrix(self) -> LEDMatrixServiceClient:
    55        if self._led_matrix is None:
    56            raise RuntimeError("LED Matrix client not initialized! Must call __aenter__() first.")
    57        return self._led_matrix
    

sound:
[pykos.services.sound.SoundServiceClient](services/sound.html#SoundServiceClient)
View Source

    
    
    59    @property
    60    def sound(self) -> SoundServiceClient:
    61        if self._sound is None:
    62            raise RuntimeError("Sound client not initialized! Must call __aenter__() first.")
    63        return self._sound
    

process_manager:
[pykos.services.process_manager.ProcessManagerServiceClient](services/process_manager.html#ProcessManagerServiceClient)
View Source

    
    
    65    @property
    66    def process_manager(self) -> ProcessManagerServiceClient:
    67        if self._process_manager is None:
    68            raise RuntimeError("Process Manager client not initialized! Must call __aenter__() first.")
    69        return self._process_manager
    

inference:
[pykos.services.inference.InferenceServiceClient](services/inference.html#InferenceServiceClient)
View Source

    
    
    71    @property
    72    def inference(self) -> InferenceServiceClient:
    73        if self._inference is None:
    74            raise RuntimeError("Inference client not initialized! Must call __aenter__() first.")
    75        return self._inference
    

sim: [pykos.services.sim.SimServiceClient](services/sim.html#SimServiceClient)
View Source

    
    
    77    @property
    78    def sim(self) -> SimServiceClient:
    79        if self._sim is None:
    80            raise RuntimeError("Sim client not initialized! Must call __aenter__() first.")
    81        return self._sim
    

async def connect(self) -> None: View Source

    
    
    83    async def connect(self) -> None:
    84        """Connect to the gRPC server and initialize service clients."""
    85        self._channel = grpc.aio.insecure_channel(f"{self.ip}:{self.port}")
    86        self._imu = IMUServiceClient(self._channel)
    87        self._actuator = ActuatorServiceClient(self._channel)
    88        self._led_matrix = LEDMatrixServiceClient(self._channel)
    89        self._sound = SoundServiceClient(self._channel)
    90        self._process_manager = ProcessManagerServiceClient(self._channel)
    91        self._inference = InferenceServiceClient(self._channel)
    92        self._sim = SimServiceClient(self._channel)
    

Connect to the gRPC server and initialize service clients.

async def close(self) -> None: View Source

    
    
    94    async def close(self) -> None:
    95        """Close the gRPC channel."""
    96        if self._channel is not None:
    97            await self._channel.close()
    

Close the gRPC channel.

