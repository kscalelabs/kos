[  pykos.services](../services.html)

## API Documentation

  * DefaultPosition
    * qpos
  * ResetRequest
    * initial_state
    * randomize
  * StepRequest
    * num_steps
    * step_size
  * SimulationParameters
    * time_scale
    * gravity
    * initial_state
  * SimServiceClient
    * SimServiceClient
    * stub
    * reset
    * set_paused
    * step
    * set_parameters
    * get_parameters

[ built with pdoc![pdoc
logo](data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20role%3D%22img%22%20aria-
label%3D%22pdoc%20logo%22%20width%3D%22300%22%20height%3D%22150%22%20viewBox%3D%22-1%200%2060%2030%22%3E%3Ctitle%3Epdoc%3C/title%3E%3Cpath%20d%3D%22M29.621%2021.293c-.011-.273-.214-.475-.511-.481a.5.5%200%200%200-.489.503l-.044%201.393c-.097.551-.695%201.215-1.566%201.704-.577.428-1.306.486-2.193.182-1.426-.617-2.467-1.654-3.304-2.487l-.173-.172a3.43%203.43%200%200%200-.365-.306.49.49%200%200%200-.286-.196c-1.718-1.06-4.931-1.47-7.353.191l-.219.15c-1.707%201.187-3.413%202.131-4.328%201.03-.02-.027-.49-.685-.141-1.763.233-.721.546-2.408.772-4.076.042-.09.067-.187.046-.288.166-1.347.277-2.625.241-3.351%201.378-1.008%202.271-2.586%202.271-4.362%200-.976-.272-1.935-.788-2.774-.057-.094-.122-.18-.184-.268.033-.167.052-.339.052-.516%200-1.477-1.202-2.679-2.679-2.679-.791%200-1.496.352-1.987.9a6.3%206.3%200%200%200-1.001.029c-.492-.564-1.207-.929-2.012-.929-1.477%200-2.679%201.202-2.679%202.679A2.65%202.65%200%200%200%20.97%206.554c-.383.747-.595%201.572-.595%202.41%200%202.311%201.507%204.29%203.635%205.107-.037.699-.147%202.27-.423%203.294l-.137.461c-.622%202.042-2.515%208.257%201.727%2010.643%201.614.908%203.06%201.248%204.317%201.248%202.665%200%204.492-1.524%205.322-2.401%201.476-1.559%202.886-1.854%206.491.82%201.877%201.393%203.514%201.753%204.861%201.068%202.223-1.713%202.811-3.867%203.399-6.374.077-.846.056-1.469.054-1.537zm-4.835%204.313c-.054.305-.156.586-.242.629-.034-.007-.131-.022-.307-.157-.145-.111-.314-.478-.456-.908.221.121.432.25.675.355.115.039.219.051.33.081zm-2.251-1.238c-.05.33-.158.648-.252.694-.022.001-.125-.018-.307-.157-.217-.166-.488-.906-.639-1.573.358.344.754.693%201.198%201.036zm-3.887-2.337c-.006-.116-.018-.231-.041-.342.635.145%201.189.368%201.599.625.097.231.166.481.174.642-.03.049-.055.101-.067.158-.046.013-.128.026-.298.004-.278-.037-.901-.57-1.367-1.087zm-1.127-.497c.116.306.176.625.12.71-.019.014-.117.045-.345.016-.206-.027-.604-.332-.986-.695.41-.051.816-.056%201.211-.031zm-4.535%201.535c.209.22.379.47.358.598-.006.041-.088.138-.351.234-.144.055-.539-.063-.979-.259a11.66%2011.66%200%200%200%20.972-.573zm.983-.664c.359-.237.738-.418%201.126-.554.25.237.479.548.457.694-.006.042-.087.138-.351.235-.174.064-.694-.105-1.232-.375zm-3.381%201.794c-.022.145-.061.29-.149.401-.133.166-.358.248-.69.251h-.002c-.133%200-.306-.26-.45-.621.417.091.854.07%201.291-.031zm-2.066-8.077a4.78%204.78%200%200%201-.775-.584c.172-.115.505-.254.88-.378l-.105.962zm-.331%202.302a10.32%2010.32%200%200%201-.828-.502c.202-.143.576-.328.984-.49l-.156.992zm-.45%202.157l-.701-.403c.214-.115.536-.249.891-.376a11.57%2011.57%200%200%201-.19.779zm-.181%201.716c.064.398.194.702.298.893-.194-.051-.435-.162-.736-.398.061-.119.224-.3.438-.495zM8.87%204.141c0%20.152-.123.276-.276.276s-.275-.124-.275-.276.123-.276.276-.276.275.124.275.276zm-.735-.389a1.15%201.15%200%200%200-.314.783%201.16%201.16%200%200%200%201.162%201.162c.457%200%20.842-.27%201.032-.653.026.117.042.238.042.362a1.68%201.68%200%200%201-1.679%201.679%201.68%201.68%200%200%201-1.679-1.679c0-.843.626-1.535%201.436-1.654zM5.059%205.406A1.68%201.68%200%200%201%203.38%207.085a1.68%201.68%200%200%201-1.679-1.679c0-.037.009-.072.011-.109.21.3.541.508.935.508a1.16%201.16%200%200%200%201.162-1.162%201.14%201.14%200%200%200-.474-.912c.015%200%20.03-.005.045-.005.926.001%201.679.754%201.679%201.68zM3.198%204.141c0%20.152-.123.276-.276.276s-.275-.124-.275-.276.123-.276.276-.276.275.124.275.276zM1.375%208.964c0-.52.103-1.035.288-1.52.466.394%201.06.64%201.717.64%201.144%200%202.116-.725%202.499-1.738.383%201.012%201.355%201.738%202.499%201.738.867%200%201.631-.421%202.121-1.062.307.605.478%201.267.478%201.942%200%202.486-2.153%204.51-4.801%204.51s-4.801-2.023-4.801-4.51zm24.342%2019.349c-.985.498-2.267.168-3.813-.979-3.073-2.281-5.453-3.199-7.813-.705-1.315%201.391-4.163%203.365-8.423.97-3.174-1.786-2.239-6.266-1.261-9.479l.146-.492c.276-1.02.395-2.457.444-3.268a6.11%206.11%200%200%200%201.18.115%206.01%206.01%200%200%200%202.536-.562l-.006.175c-.802.215-1.848.612-2.021%201.25-.079.295.021.601.274.837.219.203.415.364.598.501-.667.304-1.243.698-1.311%201.179-.02.144-.022.507.393.787.213.144.395.26.564.365-1.285.521-1.361.96-1.381%201.126-.018.142-.011.496.427.746l.854.489c-.473.389-.971.914-.999%201.429-.018.278.095.532.316.713.675.556%201.231.721%201.653.721.059%200%20.104-.014.158-.02.207.707.641%201.64%201.513%201.64h.013c.8-.008%201.236-.345%201.462-.626.173-.216.268-.457.325-.692.424.195.93.374%201.372.374.151%200%20.294-.021.423-.068.732-.27.944-.704.993-1.021.009-.061.003-.119.002-.179.266.086.538.147.789.147.15%200%20.294-.021.423-.069.542-.2.797-.489.914-.754.237.147.478.258.704.288.106.014.205.021.296.021.356%200%20.595-.101.767-.229.438.435%201.094.992%201.656%201.067.106.014.205.021.296.021a1.56%201.56%200%200%200%20.323-.035c.17.575.453%201.289.866%201.605.358.273.665.362.914.362a.99.99%200%200%200%20.421-.093%201.03%201.03%200%200%200%20.245-.164c.168.428.39.846.68%201.068.358.273.665.362.913.362a.99.99%200%200%200%20.421-.093c.317-.148.512-.448.639-.762.251.157.495.257.726.257.127%200%20.25-.024.37-.071.427-.17.706-.617.841-1.314.022-.015.047-.022.068-.038.067-.051.133-.104.196-.159-.443%201.486-1.107%202.761-2.086%203.257zM8.66%209.925a.5.5%200%201%200-1%200c0%20.653-.818%201.205-1.787%201.205s-1.787-.552-1.787-1.205a.5.5%200%201%200-1%200c0%201.216%201.25%202.205%202.787%202.205s2.787-.989%202.787-2.205zm4.4%2015.965l-.208.097c-2.661%201.258-4.708%201.436-6.086.527-1.542-1.017-1.88-3.19-1.844-4.198a.4.4%200%200%200-.385-.414c-.242-.029-.406.164-.414.385-.046%201.249.367%203.686%202.202%204.896.708.467%201.547.7%202.51.7%201.248%200%202.706-.392%204.362-1.174l.185-.086a.4.4%200%200%200%20.205-.527c-.089-.204-.326-.291-.527-.206zM9.547%202.292c.093.077.205.114.317.114a.5.5%200%200%200%20.318-.886L8.817.397a.5.5%200%200%200-.703.068.5.5%200%200%200%20.069.703l1.364%201.124zm-7.661-.065c.086%200%20.173-.022.253-.068l1.523-.893a.5.5%200%200%200-.506-.863l-1.523.892a.5.5%200%200%200-.179.685c.094.158.261.247.432.247z%22%20transform%3D%22matrix%28-1%200%200%201%2058%200%29%22%20fill%3D%22%233bb300%22/%3E%3Cpath%20d%3D%22M.3%2021.86V10.18q0-.46.02-.68.04-.22.18-.5.28-.54%201.34-.54%201.06%200%201.42.28.38.26.44.78.76-1.04%202.38-1.04%201.64%200%203.1%201.54%201.46%201.54%201.46%203.58%200%202.04-1.46%203.58-1.44%201.54-3.08%201.54-1.64%200-2.38-.92v4.04q0%20.46-.04.68-.02.22-.18.5-.14.3-.5.42-.36.12-.98.12-.62%200-1-.12-.36-.12-.52-.4-.14-.28-.18-.5-.02-.22-.02-.68zm3.96-9.42q-.46.54-.46%201.18%200%20.64.46%201.18.48.52%201.2.52.74%200%201.24-.52.52-.52.52-1.18%200-.66-.48-1.18-.48-.54-1.26-.54-.76%200-1.22.54zm14.741-8.36q.16-.3.54-.42.38-.12%201-.12.64%200%201.02.12.38.12.52.42.16.3.18.54.04.22.04.68v11.94q0%20.46-.04.7-.02.22-.18.5-.3.54-1.7.54-1.38%200-1.54-.98-.84.96-2.34.96-1.8%200-3.28-1.56-1.48-1.58-1.48-3.66%200-2.1%201.48-3.68%201.5-1.58%203.28-1.58%201.48%200%202.3%201v-4.2q0-.46.02-.68.04-.24.18-.52zm-3.24%2010.86q.52.54%201.26.54.74%200%201.22-.54.5-.54.5-1.18%200-.66-.48-1.22-.46-.56-1.26-.56-.8%200-1.28.56-.48.54-.48%201.2%200%20.66.52%201.2zm7.833-1.2q0-2.4%201.68-3.96%201.68-1.56%203.84-1.56%202.16%200%203.82%201.56%201.66%201.54%201.66%203.94%200%201.66-.86%202.96-.86%201.28-2.1%201.9-1.22.6-2.54.6-1.32%200-2.56-.64-1.24-.66-2.1-1.92-.84-1.28-.84-2.88zm4.18%201.44q.64.48%201.3.48.66%200%201.32-.5.66-.5.66-1.48%200-.98-.62-1.46-.62-.48-1.34-.48-.72%200-1.34.5-.62.5-.62%201.48%200%20.96.64%201.46zm11.412-1.44q0%20.84.56%201.32.56.46%201.18.46.64%200%201.18-.36.56-.38.9-.38.6%200%201.46%201.06.46.58.46%201.04%200%20.76-1.1%201.42-1.14.8-2.8.8-1.86%200-3.58-1.34-.82-.64-1.34-1.7-.52-1.08-.52-2.36%200-1.3.52-2.34.52-1.06%201.34-1.7%201.66-1.32%203.54-1.32.76%200%201.48.22.72.2%201.06.4l.32.2q.36.24.56.38.52.4.52.92%200%20.5-.42%201.14-.72%201.1-1.38%201.1-.38%200-1.08-.44-.36-.34-1.04-.34-.66%200-1.24.48-.58.48-.58%201.34z%22%20fill%3D%22green%22/%3E%3C/svg%3E)
](https://pdoc.dev "pdoc: Python API documentation generator")

#  [pykos](./../../pykos.html).[services](./../services.html).sim

Sim service client.

View Source

    
    
      1"""Sim service client."""
      2
      3from typing import NotRequired, TypedDict, Unpack
      4
      5import grpc
      6import grpc.aio
      7from google.protobuf.empty_pb2 import Empty
      8
      9from kos_protos import common_pb2, sim_pb2, sim_pb2_grpc
     10
     11
     12class DefaultPosition(TypedDict):
     13    qpos: list[float]
     14
     15
     16class ResetRequest(TypedDict):
     17    initial_state: NotRequired[DefaultPosition]
     18    randomize: NotRequired[bool]
     19
     20
     21class StepRequest(TypedDict):
     22    num_steps: int
     23    step_size: NotRequired[float]
     24
     25
     26class SimulationParameters(TypedDict):
     27    time_scale: NotRequired[float]
     28    gravity: NotRequired[float]
     29    initial_state: NotRequired[DefaultPosition]
     30
     31
     32class SimServiceClient:
     33    def __init__(self, channel: grpc.aio.Channel) -> None:
     34        self.stub = sim_pb2_grpc.SimulationServiceStub(channel)
     35
     36    async def reset(self, **kwargs: Unpack[ResetRequest]) -> common_pb2.ActionResponse:
     37        """Reset the simulation to its initial state.
     38
     39        Args:
     40            **kwargs: Reset parameters that may include:
     41                     initial_state: DefaultPosition to reset to
     42                     randomize: Whether to randomize the initial state
     43
     44        Example:
     45            >>> client.reset(
     46            ...     initial_state={"qpos": [0.0, 0.0, 0.0]},
     47            ...     randomize=True
     48            ... )
     49
     50        Returns:
     51            ActionResponse indicating success/failure
     52        """
     53        initial_state = None
     54        if "initial_state" in kwargs:
     55            pos = kwargs["initial_state"]
     56            initial_state = sim_pb2.DefaultPosition(qpos=pos["qpos"])
     57
     58        request = sim_pb2.ResetRequest(initial_state=initial_state, randomize=kwargs.get("randomize"))
     59        return await self.stub.Reset(request)
     60
     61    async def set_paused(self, paused: bool) -> common_pb2.ActionResponse:
     62        """Pause or unpause the simulation.
     63
     64        Args:
     65            paused: True to pause, False to unpause
     66
     67        Returns:
     68            ActionResponse indicating success/failure
     69        """
     70        request = sim_pb2.SetPausedRequest(paused=paused)
     71        return await self.stub.SetPaused(request)
     72
     73    async def step(self, num_steps: int, step_size: float | None = None) -> common_pb2.ActionResponse:
     74        """Step the simulation forward.
     75
     76        Args:
     77            num_steps: Number of simulation steps to take
     78            step_size: Optional time per step in seconds
     79
     80        Returns:
     81            ActionResponse indicating success/failure
     82        """
     83        request = sim_pb2.StepRequest(num_steps=num_steps, step_size=step_size)
     84        return await self.stub.Step(request)
     85
     86    async def set_parameters(self, **kwargs: Unpack[SimulationParameters]) -> common_pb2.ActionResponse:
     87        """Set simulation parameters.
     88
     89        Example:
     90        >>> client.set_parameters(
     91        ...     time_scale=1.0,
     92        ...     gravity=9.81,
     93        ...     initial_state={"qpos": [0.0, 0.0, 0.0]}
     94        ... )
     95
     96        Args:
     97            **kwargs: Parameters that may include:
     98                     time_scale: Simulation time scale
     99                     gravity: Gravity constant
    100                     initial_state: Default position state
    101
    102        Returns:
    103            ActionResponse indicating success/failure
    104        """
    105        initial_state = None
    106        if "initial_state" in kwargs:
    107            pos = kwargs["initial_state"]
    108            initial_state = sim_pb2.DefaultPosition(qpos=pos["qpos"])
    109
    110        params = sim_pb2.SimulationParameters(
    111            time_scale=kwargs.get("time_scale"), gravity=kwargs.get("gravity"), initial_state=initial_state
    112        )
    113        request = sim_pb2.SetParametersRequest(parameters=params)
    114        return await self.stub.SetParameters(request)
    115
    116    async def get_parameters(self) -> sim_pb2.GetParametersResponse:
    117        """Get current simulation parameters.
    118
    119        Returns:
    120            GetParametersResponse containing current parameters and any error
    121        """
    122        return await self.stub.GetParameters(Empty())
    

class DefaultPosition(typing.TypedDict): View Source

    
    
    13class DefaultPosition(TypedDict):
    14    qpos: list[float]
    

qpos: list[float]

class ResetRequest(typing.TypedDict): View Source

    
    
    17class ResetRequest(TypedDict):
    18    initial_state: NotRequired[DefaultPosition]
    19    randomize: NotRequired[bool]
    

initial_state: NotRequired[DefaultPosition]

randomize: NotRequired[bool]

class StepRequest(typing.TypedDict): View Source

    
    
    22class StepRequest(TypedDict):
    23    num_steps: int
    24    step_size: NotRequired[float]
    

num_steps: int

step_size: NotRequired[float]

class SimulationParameters(typing.TypedDict): View Source

    
    
    27class SimulationParameters(TypedDict):
    28    time_scale: NotRequired[float]
    29    gravity: NotRequired[float]
    30    initial_state: NotRequired[DefaultPosition]
    

time_scale: NotRequired[float]

gravity: NotRequired[float]

initial_state: NotRequired[DefaultPosition]

class SimServiceClient: View Source

    
    
     33class SimServiceClient:
     34    def __init__(self, channel: grpc.aio.Channel) -> None:
     35        self.stub = sim_pb2_grpc.SimulationServiceStub(channel)
     36
     37    async def reset(self, **kwargs: Unpack[ResetRequest]) -> common_pb2.ActionResponse:
     38        """Reset the simulation to its initial state.
     39
     40        Args:
     41            **kwargs: Reset parameters that may include:
     42                     initial_state: DefaultPosition to reset to
     43                     randomize: Whether to randomize the initial state
     44
     45        Example:
     46            >>> client.reset(
     47            ...     initial_state={"qpos": [0.0, 0.0, 0.0]},
     48            ...     randomize=True
     49            ... )
     50
     51        Returns:
     52            ActionResponse indicating success/failure
     53        """
     54        initial_state = None
     55        if "initial_state" in kwargs:
     56            pos = kwargs["initial_state"]
     57            initial_state = sim_pb2.DefaultPosition(qpos=pos["qpos"])
     58
     59        request = sim_pb2.ResetRequest(initial_state=initial_state, randomize=kwargs.get("randomize"))
     60        return await self.stub.Reset(request)
     61
     62    async def set_paused(self, paused: bool) -> common_pb2.ActionResponse:
     63        """Pause or unpause the simulation.
     64
     65        Args:
     66            paused: True to pause, False to unpause
     67
     68        Returns:
     69            ActionResponse indicating success/failure
     70        """
     71        request = sim_pb2.SetPausedRequest(paused=paused)
     72        return await self.stub.SetPaused(request)
     73
     74    async def step(self, num_steps: int, step_size: float | None = None) -> common_pb2.ActionResponse:
     75        """Step the simulation forward.
     76
     77        Args:
     78            num_steps: Number of simulation steps to take
     79            step_size: Optional time per step in seconds
     80
     81        Returns:
     82            ActionResponse indicating success/failure
     83        """
     84        request = sim_pb2.StepRequest(num_steps=num_steps, step_size=step_size)
     85        return await self.stub.Step(request)
     86
     87    async def set_parameters(self, **kwargs: Unpack[SimulationParameters]) -> common_pb2.ActionResponse:
     88        """Set simulation parameters.
     89
     90        Example:
     91        >>> client.set_parameters(
     92        ...     time_scale=1.0,
     93        ...     gravity=9.81,
     94        ...     initial_state={"qpos": [0.0, 0.0, 0.0]}
     95        ... )
     96
     97        Args:
     98            **kwargs: Parameters that may include:
     99                     time_scale: Simulation time scale
    100                     gravity: Gravity constant
    101                     initial_state: Default position state
    102
    103        Returns:
    104            ActionResponse indicating success/failure
    105        """
    106        initial_state = None
    107        if "initial_state" in kwargs:
    108            pos = kwargs["initial_state"]
    109            initial_state = sim_pb2.DefaultPosition(qpos=pos["qpos"])
    110
    111        params = sim_pb2.SimulationParameters(
    112            time_scale=kwargs.get("time_scale"), gravity=kwargs.get("gravity"), initial_state=initial_state
    113        )
    114        request = sim_pb2.SetParametersRequest(parameters=params)
    115        return await self.stub.SetParameters(request)
    116
    117    async def get_parameters(self) -> sim_pb2.GetParametersResponse:
    118        """Get current simulation parameters.
    119
    120        Returns:
    121            GetParametersResponse containing current parameters and any error
    122        """
    123        return await self.stub.GetParameters(Empty())
    

SimServiceClient(channel: grpc.aio._base_channel.Channel) View Source

    
    
    34    def __init__(self, channel: grpc.aio.Channel) -> None:
    35        self.stub = sim_pb2_grpc.SimulationServiceStub(channel)
    

stub

async def reset( self, **kwargs: *<class 'ResetRequest'>) ->
kos.common_pb2.ActionResponse: View Source

    
    
    37    async def reset(self, **kwargs: Unpack[ResetRequest]) -> common_pb2.ActionResponse:
    38        """Reset the simulation to its initial state.
    39
    40        Args:
    41            **kwargs: Reset parameters that may include:
    42                     initial_state: DefaultPosition to reset to
    43                     randomize: Whether to randomize the initial state
    44
    45        Example:
    46            >>> client.reset(
    47            ...     initial_state={"qpos": [0.0, 0.0, 0.0]},
    48            ...     randomize=True
    49            ... )
    50
    51        Returns:
    52            ActionResponse indicating success/failure
    53        """
    54        initial_state = None
    55        if "initial_state" in kwargs:
    56            pos = kwargs["initial_state"]
    57            initial_state = sim_pb2.DefaultPosition(qpos=pos["qpos"])
    58
    59        request = sim_pb2.ResetRequest(initial_state=initial_state, randomize=kwargs.get("randomize"))
    60        return await self.stub.Reset(request)
    

Reset the simulation to its initial state.

Args: **kwargs: Reset parameters that may include: initial_state:
DefaultPosition to reset to randomize: Whether to randomize the initial state

Example:

> > > client.reset( ... initial_state={"qpos": [0.0, 0.0, 0.0]}, ...
> randomize=True ... )

Returns: ActionResponse indicating success/failure

async def set_paused(self, paused: bool) -> kos.common_pb2.ActionResponse:
View Source

    
    
    62    async def set_paused(self, paused: bool) -> common_pb2.ActionResponse:
    63        """Pause or unpause the simulation.
    64
    65        Args:
    66            paused: True to pause, False to unpause
    67
    68        Returns:
    69            ActionResponse indicating success/failure
    70        """
    71        request = sim_pb2.SetPausedRequest(paused=paused)
    72        return await self.stub.SetPaused(request)
    

Pause or unpause the simulation.

Args: paused: True to pause, False to unpause

Returns: ActionResponse indicating success/failure

async def step( self, num_steps: int, step_size: float | None = None) -> kos.common_pb2.ActionResponse: View Source
    
    
    74    async def step(self, num_steps: int, step_size: float | None = None) -> common_pb2.ActionResponse:
    75        """Step the simulation forward.
    76
    77        Args:
    78            num_steps: Number of simulation steps to take
    79            step_size: Optional time per step in seconds
    80
    81        Returns:
    82            ActionResponse indicating success/failure
    83        """
    84        request = sim_pb2.StepRequest(num_steps=num_steps, step_size=step_size)
    85        return await self.stub.Step(request)
    

Step the simulation forward.

Args: num_steps: Number of simulation steps to take step_size: Optional time
per step in seconds

Returns: ActionResponse indicating success/failure

async def set_parameters( self, **kwargs: *<class 'SimulationParameters'>) ->
kos.common_pb2.ActionResponse: View Source

    
    
     87    async def set_parameters(self, **kwargs: Unpack[SimulationParameters]) -> common_pb2.ActionResponse:
     88        """Set simulation parameters.
     89
     90        Example:
     91        >>> client.set_parameters(
     92        ...     time_scale=1.0,
     93        ...     gravity=9.81,
     94        ...     initial_state={"qpos": [0.0, 0.0, 0.0]}
     95        ... )
     96
     97        Args:
     98            **kwargs: Parameters that may include:
     99                     time_scale: Simulation time scale
    100                     gravity: Gravity constant
    101                     initial_state: Default position state
    102
    103        Returns:
    104            ActionResponse indicating success/failure
    105        """
    106        initial_state = None
    107        if "initial_state" in kwargs:
    108            pos = kwargs["initial_state"]
    109            initial_state = sim_pb2.DefaultPosition(qpos=pos["qpos"])
    110
    111        params = sim_pb2.SimulationParameters(
    112            time_scale=kwargs.get("time_scale"), gravity=kwargs.get("gravity"), initial_state=initial_state
    113        )
    114        request = sim_pb2.SetParametersRequest(parameters=params)
    115        return await self.stub.SetParameters(request)
    

Set simulation parameters.

Example:

    
    
    >>> client.set_parameters(
    ...     time_scale=1.0,
    ...     gravity=9.81,
    ...     initial_state={"qpos": [0.0, 0.0, 0.0]}
    ... )
    

Args: **kwargs: Parameters that may include: time_scale: Simulation time scale
gravity: Gravity constant initial_state: Default position state

Returns: ActionResponse indicating success/failure

async def get_parameters(self) -> kos.sim_pb2.GetParametersResponse: View
Source

    
    
    117    async def get_parameters(self) -> sim_pb2.GetParametersResponse:
    118        """Get current simulation parameters.
    119
    120        Returns:
    121            GetParametersResponse containing current parameters and any error
    122        """
    123        return await self.stub.GetParameters(Empty())
    

Get current simulation parameters.

Returns: GetParametersResponse containing current parameters and any error

