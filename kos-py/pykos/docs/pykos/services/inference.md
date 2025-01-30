[  pykos.services](../services.html)

## API Documentation

  * ModelMetadata
    * model_name
    * model_description
    * model_version
    * model_author
  * TensorDimension
    * size
    * name
    * dynamic
  * Tensor
    * values
    * shape
  * ForwardResponse
    * outputs
    * error
  * ModelInfo
    * uid
    * metadata
    * input_specs
    * output_specs
    * description
  * GetModelsInfoResponse
    * models
    * error
  * InferenceServiceClient
    * InferenceServiceClient
    * stub
    * upload_model
    * load_models
    * unload_models
    * get_models_info
    * forward

[ built with pdoc![pdoc
logo](data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20role%3D%22img%22%20aria-
label%3D%22pdoc%20logo%22%20width%3D%22300%22%20height%3D%22150%22%20viewBox%3D%22-1%200%2060%2030%22%3E%3Ctitle%3Epdoc%3C/title%3E%3Cpath%20d%3D%22M29.621%2021.293c-.011-.273-.214-.475-.511-.481a.5.5%200%200%200-.489.503l-.044%201.393c-.097.551-.695%201.215-1.566%201.704-.577.428-1.306.486-2.193.182-1.426-.617-2.467-1.654-3.304-2.487l-.173-.172a3.43%203.43%200%200%200-.365-.306.49.49%200%200%200-.286-.196c-1.718-1.06-4.931-1.47-7.353.191l-.219.15c-1.707%201.187-3.413%202.131-4.328%201.03-.02-.027-.49-.685-.141-1.763.233-.721.546-2.408.772-4.076.042-.09.067-.187.046-.288.166-1.347.277-2.625.241-3.351%201.378-1.008%202.271-2.586%202.271-4.362%200-.976-.272-1.935-.788-2.774-.057-.094-.122-.18-.184-.268.033-.167.052-.339.052-.516%200-1.477-1.202-2.679-2.679-2.679-.791%200-1.496.352-1.987.9a6.3%206.3%200%200%200-1.001.029c-.492-.564-1.207-.929-2.012-.929-1.477%200-2.679%201.202-2.679%202.679A2.65%202.65%200%200%200%20.97%206.554c-.383.747-.595%201.572-.595%202.41%200%202.311%201.507%204.29%203.635%205.107-.037.699-.147%202.27-.423%203.294l-.137.461c-.622%202.042-2.515%208.257%201.727%2010.643%201.614.908%203.06%201.248%204.317%201.248%202.665%200%204.492-1.524%205.322-2.401%201.476-1.559%202.886-1.854%206.491.82%201.877%201.393%203.514%201.753%204.861%201.068%202.223-1.713%202.811-3.867%203.399-6.374.077-.846.056-1.469.054-1.537zm-4.835%204.313c-.054.305-.156.586-.242.629-.034-.007-.131-.022-.307-.157-.145-.111-.314-.478-.456-.908.221.121.432.25.675.355.115.039.219.051.33.081zm-2.251-1.238c-.05.33-.158.648-.252.694-.022.001-.125-.018-.307-.157-.217-.166-.488-.906-.639-1.573.358.344.754.693%201.198%201.036zm-3.887-2.337c-.006-.116-.018-.231-.041-.342.635.145%201.189.368%201.599.625.097.231.166.481.174.642-.03.049-.055.101-.067.158-.046.013-.128.026-.298.004-.278-.037-.901-.57-1.367-1.087zm-1.127-.497c.116.306.176.625.12.71-.019.014-.117.045-.345.016-.206-.027-.604-.332-.986-.695.41-.051.816-.056%201.211-.031zm-4.535%201.535c.209.22.379.47.358.598-.006.041-.088.138-.351.234-.144.055-.539-.063-.979-.259a11.66%2011.66%200%200%200%20.972-.573zm.983-.664c.359-.237.738-.418%201.126-.554.25.237.479.548.457.694-.006.042-.087.138-.351.235-.174.064-.694-.105-1.232-.375zm-3.381%201.794c-.022.145-.061.29-.149.401-.133.166-.358.248-.69.251h-.002c-.133%200-.306-.26-.45-.621.417.091.854.07%201.291-.031zm-2.066-8.077a4.78%204.78%200%200%201-.775-.584c.172-.115.505-.254.88-.378l-.105.962zm-.331%202.302a10.32%2010.32%200%200%201-.828-.502c.202-.143.576-.328.984-.49l-.156.992zm-.45%202.157l-.701-.403c.214-.115.536-.249.891-.376a11.57%2011.57%200%200%201-.19.779zm-.181%201.716c.064.398.194.702.298.893-.194-.051-.435-.162-.736-.398.061-.119.224-.3.438-.495zM8.87%204.141c0%20.152-.123.276-.276.276s-.275-.124-.275-.276.123-.276.276-.276.275.124.275.276zm-.735-.389a1.15%201.15%200%200%200-.314.783%201.16%201.16%200%200%200%201.162%201.162c.457%200%20.842-.27%201.032-.653.026.117.042.238.042.362a1.68%201.68%200%200%201-1.679%201.679%201.68%201.68%200%200%201-1.679-1.679c0-.843.626-1.535%201.436-1.654zM5.059%205.406A1.68%201.68%200%200%201%203.38%207.085a1.68%201.68%200%200%201-1.679-1.679c0-.037.009-.072.011-.109.21.3.541.508.935.508a1.16%201.16%200%200%200%201.162-1.162%201.14%201.14%200%200%200-.474-.912c.015%200%20.03-.005.045-.005.926.001%201.679.754%201.679%201.68zM3.198%204.141c0%20.152-.123.276-.276.276s-.275-.124-.275-.276.123-.276.276-.276.275.124.275.276zM1.375%208.964c0-.52.103-1.035.288-1.52.466.394%201.06.64%201.717.64%201.144%200%202.116-.725%202.499-1.738.383%201.012%201.355%201.738%202.499%201.738.867%200%201.631-.421%202.121-1.062.307.605.478%201.267.478%201.942%200%202.486-2.153%204.51-4.801%204.51s-4.801-2.023-4.801-4.51zm24.342%2019.349c-.985.498-2.267.168-3.813-.979-3.073-2.281-5.453-3.199-7.813-.705-1.315%201.391-4.163%203.365-8.423.97-3.174-1.786-2.239-6.266-1.261-9.479l.146-.492c.276-1.02.395-2.457.444-3.268a6.11%206.11%200%200%200%201.18.115%206.01%206.01%200%200%200%202.536-.562l-.006.175c-.802.215-1.848.612-2.021%201.25-.079.295.021.601.274.837.219.203.415.364.598.501-.667.304-1.243.698-1.311%201.179-.02.144-.022.507.393.787.213.144.395.26.564.365-1.285.521-1.361.96-1.381%201.126-.018.142-.011.496.427.746l.854.489c-.473.389-.971.914-.999%201.429-.018.278.095.532.316.713.675.556%201.231.721%201.653.721.059%200%20.104-.014.158-.02.207.707.641%201.64%201.513%201.64h.013c.8-.008%201.236-.345%201.462-.626.173-.216.268-.457.325-.692.424.195.93.374%201.372.374.151%200%20.294-.021.423-.068.732-.27.944-.704.993-1.021.009-.061.003-.119.002-.179.266.086.538.147.789.147.15%200%20.294-.021.423-.069.542-.2.797-.489.914-.754.237.147.478.258.704.288.106.014.205.021.296.021.356%200%20.595-.101.767-.229.438.435%201.094.992%201.656%201.067.106.014.205.021.296.021a1.56%201.56%200%200%200%20.323-.035c.17.575.453%201.289.866%201.605.358.273.665.362.914.362a.99.99%200%200%200%20.421-.093%201.03%201.03%200%200%200%20.245-.164c.168.428.39.846.68%201.068.358.273.665.362.913.362a.99.99%200%200%200%20.421-.093c.317-.148.512-.448.639-.762.251.157.495.257.726.257.127%200%20.25-.024.37-.071.427-.17.706-.617.841-1.314.022-.015.047-.022.068-.038.067-.051.133-.104.196-.159-.443%201.486-1.107%202.761-2.086%203.257zM8.66%209.925a.5.5%200%201%200-1%200c0%20.653-.818%201.205-1.787%201.205s-1.787-.552-1.787-1.205a.5.5%200%201%200-1%200c0%201.216%201.25%202.205%202.787%202.205s2.787-.989%202.787-2.205zm4.4%2015.965l-.208.097c-2.661%201.258-4.708%201.436-6.086.527-1.542-1.017-1.88-3.19-1.844-4.198a.4.4%200%200%200-.385-.414c-.242-.029-.406.164-.414.385-.046%201.249.367%203.686%202.202%204.896.708.467%201.547.7%202.51.7%201.248%200%202.706-.392%204.362-1.174l.185-.086a.4.4%200%200%200%20.205-.527c-.089-.204-.326-.291-.527-.206zM9.547%202.292c.093.077.205.114.317.114a.5.5%200%200%200%20.318-.886L8.817.397a.5.5%200%200%200-.703.068.5.5%200%200%200%20.069.703l1.364%201.124zm-7.661-.065c.086%200%20.173-.022.253-.068l1.523-.893a.5.5%200%200%200-.506-.863l-1.523.892a.5.5%200%200%200-.179.685c.094.158.261.247.432.247z%22%20transform%3D%22matrix%28-1%200%200%201%2058%200%29%22%20fill%3D%22%233bb300%22/%3E%3Cpath%20d%3D%22M.3%2021.86V10.18q0-.46.02-.68.04-.22.18-.5.28-.54%201.34-.54%201.06%200%201.42.28.38.26.44.78.76-1.04%202.38-1.04%201.64%200%203.1%201.54%201.46%201.54%201.46%203.58%200%202.04-1.46%203.58-1.44%201.54-3.08%201.54-1.64%200-2.38-.92v4.04q0%20.46-.04.68-.02.22-.18.5-.14.3-.5.42-.36.12-.98.12-.62%200-1-.12-.36-.12-.52-.4-.14-.28-.18-.5-.02-.22-.02-.68zm3.96-9.42q-.46.54-.46%201.18%200%20.64.46%201.18.48.52%201.2.52.74%200%201.24-.52.52-.52.52-1.18%200-.66-.48-1.18-.48-.54-1.26-.54-.76%200-1.22.54zm14.741-8.36q.16-.3.54-.42.38-.12%201-.12.64%200%201.02.12.38.12.52.42.16.3.18.54.04.22.04.68v11.94q0%20.46-.04.7-.02.22-.18.5-.3.54-1.7.54-1.38%200-1.54-.98-.84.96-2.34.96-1.8%200-3.28-1.56-1.48-1.58-1.48-3.66%200-2.1%201.48-3.68%201.5-1.58%203.28-1.58%201.48%200%202.3%201v-4.2q0-.46.02-.68.04-.24.18-.52zm-3.24%2010.86q.52.54%201.26.54.74%200%201.22-.54.5-.54.5-1.18%200-.66-.48-1.22-.46-.56-1.26-.56-.8%200-1.28.56-.48.54-.48%201.2%200%20.66.52%201.2zm7.833-1.2q0-2.4%201.68-3.96%201.68-1.56%203.84-1.56%202.16%200%203.82%201.56%201.66%201.54%201.66%203.94%200%201.66-.86%202.96-.86%201.28-2.1%201.9-1.22.6-2.54.6-1.32%200-2.56-.64-1.24-.66-2.1-1.92-.84-1.28-.84-2.88zm4.18%201.44q.64.48%201.3.48.66%200%201.32-.5.66-.5.66-1.48%200-.98-.62-1.46-.62-.48-1.34-.48-.72%200-1.34.5-.62.5-.62%201.48%200%20.96.64%201.46zm11.412-1.44q0%20.84.56%201.32.56.46%201.18.46.64%200%201.18-.36.56-.38.9-.38.6%200%201.46%201.06.46.58.46%201.04%200%20.76-1.1%201.42-1.14.8-2.8.8-1.86%200-3.58-1.34-.82-.64-1.34-1.7-.52-1.08-.52-2.36%200-1.3.52-2.34.52-1.06%201.34-1.7%201.66-1.32%203.54-1.32.76%200%201.48.22.72.2%201.06.4l.32.2q.36.24.56.38.52.4.52.92%200%20.5-.42%201.14-.72%201.1-1.38%201.1-.38%200-1.08-.44-.36-.34-1.04-.34-.66%200-1.24.48-.58.48-.58%201.34z%22%20fill%3D%22green%22/%3E%3C/svg%3E)
](https://pdoc.dev "pdoc: Python API documentation generator")

#  [pykos](./../../pykos.html).[services](./../services.html).inference

Inference service client.

View Source

    
    
      1"""Inference service client."""
      2
      3from typing import NotRequired, TypedDict
      4
      5import grpc
      6import grpc.aio
      7
      8from kos_protos import common_pb2, inference_pb2, inference_pb2_grpc
      9
     10
     11class ModelMetadata(TypedDict):
     12    """Model metadata for uploading models.
     13
     14    All fields are optional and can be used to provide additional information about the model.
     15    """
     16
     17    model_name: NotRequired[str | None]
     18    model_description: NotRequired[str | None]
     19    model_version: NotRequired[str | None]
     20    model_author: NotRequired[str | None]
     21
     22
     23class TensorDimension(TypedDict):
     24    """Information about a tensor dimension.
     25
     26    Args:
     27        size: Size of this dimension
     28        name: Name of this dimension (e.g., "batch", "channels", "height")
     29        dynamic: Whether this dimension can vary (e.g., batch size)
     30    """
     31
     32    size: int
     33    name: str
     34    dynamic: bool
     35
     36
     37class Tensor(TypedDict):
     38    """A tensor containing data.
     39
     40    Args:
     41        values: Tensor values in row-major order
     42        shape: List of dimension information
     43    """
     44
     45    values: list[float]
     46    shape: list[TensorDimension]
     47
     48
     49class ForwardResponse(TypedDict):
     50    """Response from running model inference.
     51
     52    Args:
     53        outputs: Dictionary mapping tensor names to output tensors
     54        error: Optional error information if inference failed
     55    """
     56
     57    outputs: dict[str, Tensor]
     58    error: NotRequired[common_pb2.Error | None]
     59
     60
     61class ModelInfo(TypedDict):
     62    """Information about a model.
     63
     64    Args:
     65        uid: Model UID (assigned by server)
     66        metadata: Model metadata
     67        input_specs: Expected input tensor specifications
     68        output_specs: Expected output tensor specifications
     69        description: str
     70    """
     71
     72    uid: str
     73    metadata: ModelMetadata
     74    input_specs: dict[str, Tensor]
     75    output_specs: dict[str, Tensor]
     76    description: str
     77
     78
     79class GetModelsInfoResponse(TypedDict):
     80    """Response containing information about available models."""
     81
     82    models: list[ModelInfo]
     83    error: NotRequired[common_pb2.Error | None]
     84
     85
     86class InferenceServiceClient:
     87    """Client for the InferenceService.
     88
     89    This service allows uploading models and running inference on them.
     90    """
     91
     92    def __init__(self, channel: grpc.aio.Channel) -> None:
     93        """Initialize the inference service client.
     94
     95        Args:
     96            channel: gRPC channel to use for communication.
     97        """
     98        self.stub = inference_pb2_grpc.InferenceServiceStub(channel)
     99
    100    async def upload_model(
    101        self, model_data: bytes, metadata: ModelMetadata | None = None
    102    ) -> inference_pb2.UploadModelResponse:
    103        """Upload a model to the robot.
    104
    105        Example:
    106        >>> client.upload_model(model_data,
    107        ... metadata={"model_name": "MyModel",
    108        ... "model_description": "A model for inference",
    109        ... "model_version": "1.0.0",
    110        ... "model_author": "John Doe"})
    111
    112        Args:
    113            model_data: The binary model data to upload.
    114            metadata: Optional metadata about the model that can include:
    115                     model_name: Name of the model
    116                     model_description: Description of the model
    117                     model_version: Version of the model
    118                     model_author: Author of the model
    119
    120        Returns:
    121            UploadModelResponse containing the model UID and any error information.
    122        """
    123        proto_metadata = None
    124        if metadata is not None:
    125            proto_metadata = inference_pb2.ModelMetadata(**metadata)
    126        request = inference_pb2.UploadModelRequest(model=model_data, metadata=proto_metadata)
    127        return await self.stub.UploadModel(request)
    128
    129    async def load_models(self, uids: list[str]) -> inference_pb2.LoadModelsResponse:
    130        """Load models from the robot's filesystem.
    131
    132        Args:
    133            uids: List of model UIDs to load.
    134
    135        Returns:
    136            LoadModelsResponse containing information about the loaded models.
    137        """
    138        request = inference_pb2.ModelUids(uids=uids)
    139        return await self.stub.LoadModels(request)
    140
    141    async def unload_models(self, uids: list[str]) -> common_pb2.ActionResponse:
    142        """Unload models from the robot's filesystem.
    143
    144        Args:
    145            uids: List of model UIDs to unload.
    146
    147        Returns:
    148            ActionResponse indicating success/failure of the unload operation.
    149        """
    150        request = inference_pb2.ModelUids(uids=uids)
    151        return await self.stub.UnloadModels(request)
    152
    153    async def get_models_info(self, model_uids: list[str] | None = None) -> GetModelsInfoResponse:
    154        """Get information about available models.
    155
    156        Args:
    157            model_uids: Optional list of specific model UIDs to get info for.
    158                       If None, returns info for all models.
    159
    160        Returns:
    161            GetModelsInfoResponse containing:
    162                models: List of ModelInfo objects
    163                error: Optional error information if fetching failed
    164        """
    165        if model_uids is not None:
    166            request = inference_pb2.GetModelsInfoRequest(model_uids=inference_pb2.ModelUids(uids=model_uids))
    167        else:
    168            request = inference_pb2.GetModelsInfoRequest(all=True)
    169
    170        response = await self.stub.GetModelsInfo(request)
    171
    172        return GetModelsInfoResponse(
    173            models=[
    174                ModelInfo(
    175                    uid=model.uid,
    176                    metadata=ModelMetadata(
    177                        model_name=model.metadata.model_name if model.metadata.HasField("model_name") else None,
    178                        model_description=(
    179                            model.metadata.model_description if model.metadata.HasField("model_description") else None
    180                        ),
    181                        model_version=(
    182                            model.metadata.model_version if model.metadata.HasField("model_version") else None
    183                        ),
    184                        model_author=model.metadata.model_author if model.metadata.HasField("model_author") else None,
    185                    ),
    186                    input_specs={
    187                        name: Tensor(
    188                            values=list(tensor.values),
    189                            shape=[
    190                                TensorDimension(size=dim.size, name=dim.name, dynamic=dim.dynamic)
    191                                for dim in tensor.shape
    192                            ],
    193                        )
    194                        for name, tensor in model.input_specs.items()
    195                    },
    196                    output_specs={
    197                        name: Tensor(
    198                            values=list(tensor.values),
    199                            shape=[
    200                                TensorDimension(size=dim.size, name=dim.name, dynamic=dim.dynamic)
    201                                for dim in tensor.shape
    202                            ],
    203                        )
    204                        for name, tensor in model.output_specs.items()
    205                    },
    206                    description=model.description,
    207                )
    208                for model in response.models
    209            ],
    210            error=response.error if response.HasField("error") else None,
    211        )
    212
    213    async def forward(self, model_uid: str, inputs: dict[str, Tensor]) -> ForwardResponse:
    214        """Run inference using a specified model.
    215
    216        Args:
    217            model_uid: The UID of the model to use for inference.
    218            inputs: Dictionary mapping tensor names to tensors.
    219
    220        Returns:
    221            ForwardResponse containing:
    222                outputs: Dictionary mapping tensor names to output tensors
    223                error: Optional error information if inference failed
    224        """
    225        tensor_inputs = {}
    226        for name, tensor in inputs.items():
    227            shape = [
    228                inference_pb2.Tensor.Dimension(size=dim["size"], name=dim["name"], dynamic=dim["dynamic"])
    229                for dim in tensor["shape"]
    230            ]
    231            proto_tensor = inference_pb2.Tensor(values=tensor["values"], shape=shape)
    232            tensor_inputs[name] = proto_tensor
    233
    234        response = await self.stub.Forward(inference_pb2.ForwardRequest(model_uid=model_uid, inputs=tensor_inputs))
    235
    236        return ForwardResponse(
    237            outputs={
    238                name: Tensor(
    239                    values=list(tensor.values),
    240                    shape=[TensorDimension(size=dim.size, name=dim.name, dynamic=dim.dynamic) for dim in tensor.shape],
    241                )
    242                for name, tensor in response.outputs.items()
    243            },
    244            error=response.error if response.HasField("error") else None,
    245        )
    

class ModelMetadata(typing.TypedDict): View Source

    
    
    12class ModelMetadata(TypedDict):
    13    """Model metadata for uploading models.
    14
    15    All fields are optional and can be used to provide additional information about the model.
    16    """
    17
    18    model_name: NotRequired[str | None]
    19    model_description: NotRequired[str | None]
    20    model_version: NotRequired[str | None]
    21    model_author: NotRequired[str | None]
    

Model metadata for uploading models.

All fields are optional and can be used to provide additional information
about the model.

model_name: NotRequired[str | None]

model_description: NotRequired[str | None]

model_version: NotRequired[str | None]

model_author: NotRequired[str | None]

class TensorDimension(typing.TypedDict): View Source

    
    
    24class TensorDimension(TypedDict):
    25    """Information about a tensor dimension.
    26
    27    Args:
    28        size: Size of this dimension
    29        name: Name of this dimension (e.g., "batch", "channels", "height")
    30        dynamic: Whether this dimension can vary (e.g., batch size)
    31    """
    32
    33    size: int
    34    name: str
    35    dynamic: bool
    

Information about a tensor dimension.

Args: size: Size of this dimension name: Name of this dimension (e.g.,
"batch", "channels", "height") dynamic: Whether this dimension can vary (e.g.,
batch size)

size: int

name: str

dynamic: bool

class Tensor(typing.TypedDict): View Source

    
    
    38class Tensor(TypedDict):
    39    """A tensor containing data.
    40
    41    Args:
    42        values: Tensor values in row-major order
    43        shape: List of dimension information
    44    """
    45
    46    values: list[float]
    47    shape: list[TensorDimension]
    

A tensor containing data.

Args: values: Tensor values in row-major order shape: List of dimension
information

values: list[float]

shape: list[TensorDimension]

class ForwardResponse(typing.TypedDict): View Source

    
    
    50class ForwardResponse(TypedDict):
    51    """Response from running model inference.
    52
    53    Args:
    54        outputs: Dictionary mapping tensor names to output tensors
    55        error: Optional error information if inference failed
    56    """
    57
    58    outputs: dict[str, Tensor]
    59    error: NotRequired[common_pb2.Error | None]
    

Response from running model inference.

Args: outputs: Dictionary mapping tensor names to output tensors error:
Optional error information if inference failed

outputs: dict[str, Tensor]

error: NotRequired[kos.common_pb2.Error | None]

class ModelInfo(typing.TypedDict): View Source

    
    
    62class ModelInfo(TypedDict):
    63    """Information about a model.
    64
    65    Args:
    66        uid: Model UID (assigned by server)
    67        metadata: Model metadata
    68        input_specs: Expected input tensor specifications
    69        output_specs: Expected output tensor specifications
    70        description: str
    71    """
    72
    73    uid: str
    74    metadata: ModelMetadata
    75    input_specs: dict[str, Tensor]
    76    output_specs: dict[str, Tensor]
    77    description: str
    

Information about a model.

Args: uid: Model UID (assigned by server) metadata: Model metadata
input_specs: Expected input tensor specifications output_specs: Expected
output tensor specifications description: str

uid: str

metadata: ModelMetadata

input_specs: dict[str, Tensor]

output_specs: dict[str, Tensor]

description: str

class GetModelsInfoResponse(typing.TypedDict): View Source

    
    
    80class GetModelsInfoResponse(TypedDict):
    81    """Response containing information about available models."""
    82
    83    models: list[ModelInfo]
    84    error: NotRequired[common_pb2.Error | None]
    

Response containing information about available models.

models: list[ModelInfo]

error: NotRequired[kos.common_pb2.Error | None]

class InferenceServiceClient: View Source

    
    
     87class InferenceServiceClient:
     88    """Client for the InferenceService.
     89
     90    This service allows uploading models and running inference on them.
     91    """
     92
     93    def __init__(self, channel: grpc.aio.Channel) -> None:
     94        """Initialize the inference service client.
     95
     96        Args:
     97            channel: gRPC channel to use for communication.
     98        """
     99        self.stub = inference_pb2_grpc.InferenceServiceStub(channel)
    100
    101    async def upload_model(
    102        self, model_data: bytes, metadata: ModelMetadata | None = None
    103    ) -> inference_pb2.UploadModelResponse:
    104        """Upload a model to the robot.
    105
    106        Example:
    107        >>> client.upload_model(model_data,
    108        ... metadata={"model_name": "MyModel",
    109        ... "model_description": "A model for inference",
    110        ... "model_version": "1.0.0",
    111        ... "model_author": "John Doe"})
    112
    113        Args:
    114            model_data: The binary model data to upload.
    115            metadata: Optional metadata about the model that can include:
    116                     model_name: Name of the model
    117                     model_description: Description of the model
    118                     model_version: Version of the model
    119                     model_author: Author of the model
    120
    121        Returns:
    122            UploadModelResponse containing the model UID and any error information.
    123        """
    124        proto_metadata = None
    125        if metadata is not None:
    126            proto_metadata = inference_pb2.ModelMetadata(**metadata)
    127        request = inference_pb2.UploadModelRequest(model=model_data, metadata=proto_metadata)
    128        return await self.stub.UploadModel(request)
    129
    130    async def load_models(self, uids: list[str]) -> inference_pb2.LoadModelsResponse:
    131        """Load models from the robot's filesystem.
    132
    133        Args:
    134            uids: List of model UIDs to load.
    135
    136        Returns:
    137            LoadModelsResponse containing information about the loaded models.
    138        """
    139        request = inference_pb2.ModelUids(uids=uids)
    140        return await self.stub.LoadModels(request)
    141
    142    async def unload_models(self, uids: list[str]) -> common_pb2.ActionResponse:
    143        """Unload models from the robot's filesystem.
    144
    145        Args:
    146            uids: List of model UIDs to unload.
    147
    148        Returns:
    149            ActionResponse indicating success/failure of the unload operation.
    150        """
    151        request = inference_pb2.ModelUids(uids=uids)
    152        return await self.stub.UnloadModels(request)
    153
    154    async def get_models_info(self, model_uids: list[str] | None = None) -> GetModelsInfoResponse:
    155        """Get information about available models.
    156
    157        Args:
    158            model_uids: Optional list of specific model UIDs to get info for.
    159                       If None, returns info for all models.
    160
    161        Returns:
    162            GetModelsInfoResponse containing:
    163                models: List of ModelInfo objects
    164                error: Optional error information if fetching failed
    165        """
    166        if model_uids is not None:
    167            request = inference_pb2.GetModelsInfoRequest(model_uids=inference_pb2.ModelUids(uids=model_uids))
    168        else:
    169            request = inference_pb2.GetModelsInfoRequest(all=True)
    170
    171        response = await self.stub.GetModelsInfo(request)
    172
    173        return GetModelsInfoResponse(
    174            models=[
    175                ModelInfo(
    176                    uid=model.uid,
    177                    metadata=ModelMetadata(
    178                        model_name=model.metadata.model_name if model.metadata.HasField("model_name") else None,
    179                        model_description=(
    180                            model.metadata.model_description if model.metadata.HasField("model_description") else None
    181                        ),
    182                        model_version=(
    183                            model.metadata.model_version if model.metadata.HasField("model_version") else None
    184                        ),
    185                        model_author=model.metadata.model_author if model.metadata.HasField("model_author") else None,
    186                    ),
    187                    input_specs={
    188                        name: Tensor(
    189                            values=list(tensor.values),
    190                            shape=[
    191                                TensorDimension(size=dim.size, name=dim.name, dynamic=dim.dynamic)
    192                                for dim in tensor.shape
    193                            ],
    194                        )
    195                        for name, tensor in model.input_specs.items()
    196                    },
    197                    output_specs={
    198                        name: Tensor(
    199                            values=list(tensor.values),
    200                            shape=[
    201                                TensorDimension(size=dim.size, name=dim.name, dynamic=dim.dynamic)
    202                                for dim in tensor.shape
    203                            ],
    204                        )
    205                        for name, tensor in model.output_specs.items()
    206                    },
    207                    description=model.description,
    208                )
    209                for model in response.models
    210            ],
    211            error=response.error if response.HasField("error") else None,
    212        )
    213
    214    async def forward(self, model_uid: str, inputs: dict[str, Tensor]) -> ForwardResponse:
    215        """Run inference using a specified model.
    216
    217        Args:
    218            model_uid: The UID of the model to use for inference.
    219            inputs: Dictionary mapping tensor names to tensors.
    220
    221        Returns:
    222            ForwardResponse containing:
    223                outputs: Dictionary mapping tensor names to output tensors
    224                error: Optional error information if inference failed
    225        """
    226        tensor_inputs = {}
    227        for name, tensor in inputs.items():
    228            shape = [
    229                inference_pb2.Tensor.Dimension(size=dim["size"], name=dim["name"], dynamic=dim["dynamic"])
    230                for dim in tensor["shape"]
    231            ]
    232            proto_tensor = inference_pb2.Tensor(values=tensor["values"], shape=shape)
    233            tensor_inputs[name] = proto_tensor
    234
    235        response = await self.stub.Forward(inference_pb2.ForwardRequest(model_uid=model_uid, inputs=tensor_inputs))
    236
    237        return ForwardResponse(
    238            outputs={
    239                name: Tensor(
    240                    values=list(tensor.values),
    241                    shape=[TensorDimension(size=dim.size, name=dim.name, dynamic=dim.dynamic) for dim in tensor.shape],
    242                )
    243                for name, tensor in response.outputs.items()
    244            },
    245            error=response.error if response.HasField("error") else None,
    246        )
    

Client for the InferenceService.

This service allows uploading models and running inference on them.

InferenceServiceClient(channel: grpc.aio._base_channel.Channel) View Source

    
    
    93    def __init__(self, channel: grpc.aio.Channel) -> None:
    94        """Initialize the inference service client.
    95
    96        Args:
    97            channel: gRPC channel to use for communication.
    98        """
    99        self.stub = inference_pb2_grpc.InferenceServiceStub(channel)
    

Initialize the inference service client.

Args: channel: gRPC channel to use for communication.

stub

async def upload_model( self, model_data: bytes, metadata: ModelMetadata | None = None) -> kos.inference_pb2.UploadModelResponse: View Source
    
    
    101    async def upload_model(
    102        self, model_data: bytes, metadata: ModelMetadata | None = None
    103    ) -> inference_pb2.UploadModelResponse:
    104        """Upload a model to the robot.
    105
    106        Example:
    107        >>> client.upload_model(model_data,
    108        ... metadata={"model_name": "MyModel",
    109        ... "model_description": "A model for inference",
    110        ... "model_version": "1.0.0",
    111        ... "model_author": "John Doe"})
    112
    113        Args:
    114            model_data: The binary model data to upload.
    115            metadata: Optional metadata about the model that can include:
    116                     model_name: Name of the model
    117                     model_description: Description of the model
    118                     model_version: Version of the model
    119                     model_author: Author of the model
    120
    121        Returns:
    122            UploadModelResponse containing the model UID and any error information.
    123        """
    124        proto_metadata = None
    125        if metadata is not None:
    126            proto_metadata = inference_pb2.ModelMetadata(**metadata)
    127        request = inference_pb2.UploadModelRequest(model=model_data, metadata=proto_metadata)
    128        return await self.stub.UploadModel(request)
    

Upload a model to the robot.

Example:

    
    
    >>> client.upload_model(model_data,
    ... metadata={"model_name": "MyModel",
    ... "model_description": "A model for inference",
    ... "model_version": "1.0.0",
    ... "model_author": "John Doe"})
    

Args: model_data: The binary model data to upload. metadata: Optional metadata
about the model that can include: model_name: Name of the model
model_description: Description of the model model_version: Version of the
model model_author: Author of the model

Returns: UploadModelResponse containing the model UID and any error
information.

async def load_models(self, uids: list[str]) ->
kos.inference_pb2.LoadModelsResponse: View Source

    
    
    130    async def load_models(self, uids: list[str]) -> inference_pb2.LoadModelsResponse:
    131        """Load models from the robot's filesystem.
    132
    133        Args:
    134            uids: List of model UIDs to load.
    135
    136        Returns:
    137            LoadModelsResponse containing information about the loaded models.
    138        """
    139        request = inference_pb2.ModelUids(uids=uids)
    140        return await self.stub.LoadModels(request)
    

Load models from the robot's filesystem.

Args: uids: List of model UIDs to load.

Returns: LoadModelsResponse containing information about the loaded models.

async def unload_models(self, uids: list[str]) ->
kos.common_pb2.ActionResponse: View Source

    
    
    142    async def unload_models(self, uids: list[str]) -> common_pb2.ActionResponse:
    143        """Unload models from the robot's filesystem.
    144
    145        Args:
    146            uids: List of model UIDs to unload.
    147
    148        Returns:
    149            ActionResponse indicating success/failure of the unload operation.
    150        """
    151        request = inference_pb2.ModelUids(uids=uids)
    152        return await self.stub.UnloadModels(request)
    

Unload models from the robot's filesystem.

Args: uids: List of model UIDs to unload.

Returns: ActionResponse indicating success/failure of the unload operation.

async def get_models_info( self, model_uids: list[str] | None = None) -> GetModelsInfoResponse: View Source
    
    
    154    async def get_models_info(self, model_uids: list[str] | None = None) -> GetModelsInfoResponse:
    155        """Get information about available models.
    156
    157        Args:
    158            model_uids: Optional list of specific model UIDs to get info for.
    159                       If None, returns info for all models.
    160
    161        Returns:
    162            GetModelsInfoResponse containing:
    163                models: List of ModelInfo objects
    164                error: Optional error information if fetching failed
    165        """
    166        if model_uids is not None:
    167            request = inference_pb2.GetModelsInfoRequest(model_uids=inference_pb2.ModelUids(uids=model_uids))
    168        else:
    169            request = inference_pb2.GetModelsInfoRequest(all=True)
    170
    171        response = await self.stub.GetModelsInfo(request)
    172
    173        return GetModelsInfoResponse(
    174            models=[
    175                ModelInfo(
    176                    uid=model.uid,
    177                    metadata=ModelMetadata(
    178                        model_name=model.metadata.model_name if model.metadata.HasField("model_name") else None,
    179                        model_description=(
    180                            model.metadata.model_description if model.metadata.HasField("model_description") else None
    181                        ),
    182                        model_version=(
    183                            model.metadata.model_version if model.metadata.HasField("model_version") else None
    184                        ),
    185                        model_author=model.metadata.model_author if model.metadata.HasField("model_author") else None,
    186                    ),
    187                    input_specs={
    188                        name: Tensor(
    189                            values=list(tensor.values),
    190                            shape=[
    191                                TensorDimension(size=dim.size, name=dim.name, dynamic=dim.dynamic)
    192                                for dim in tensor.shape
    193                            ],
    194                        )
    195                        for name, tensor in model.input_specs.items()
    196                    },
    197                    output_specs={
    198                        name: Tensor(
    199                            values=list(tensor.values),
    200                            shape=[
    201                                TensorDimension(size=dim.size, name=dim.name, dynamic=dim.dynamic)
    202                                for dim in tensor.shape
    203                            ],
    204                        )
    205                        for name, tensor in model.output_specs.items()
    206                    },
    207                    description=model.description,
    208                )
    209                for model in response.models
    210            ],
    211            error=response.error if response.HasField("error") else None,
    212        )
    

Get information about available models.

Args: model_uids: Optional list of specific model UIDs to get info for. If
None, returns info for all models.

Returns: GetModelsInfoResponse containing: models: List of ModelInfo objects
error: Optional error information if fetching failed

async def forward( self, model_uid: str, inputs: dict[str, Tensor]) ->
ForwardResponse: View Source

    
    
    214    async def forward(self, model_uid: str, inputs: dict[str, Tensor]) -> ForwardResponse:
    215        """Run inference using a specified model.
    216
    217        Args:
    218            model_uid: The UID of the model to use for inference.
    219            inputs: Dictionary mapping tensor names to tensors.
    220
    221        Returns:
    222            ForwardResponse containing:
    223                outputs: Dictionary mapping tensor names to output tensors
    224                error: Optional error information if inference failed
    225        """
    226        tensor_inputs = {}
    227        for name, tensor in inputs.items():
    228            shape = [
    229                inference_pb2.Tensor.Dimension(size=dim["size"], name=dim["name"], dynamic=dim["dynamic"])
    230                for dim in tensor["shape"]
    231            ]
    232            proto_tensor = inference_pb2.Tensor(values=tensor["values"], shape=shape)
    233            tensor_inputs[name] = proto_tensor
    234
    235        response = await self.stub.Forward(inference_pb2.ForwardRequest(model_uid=model_uid, inputs=tensor_inputs))
    236
    237        return ForwardResponse(
    238            outputs={
    239                name: Tensor(
    240                    values=list(tensor.values),
    241                    shape=[TensorDimension(size=dim.size, name=dim.name, dynamic=dim.dynamic) for dim in tensor.shape],
    242                )
    243                for name, tensor in response.outputs.items()
    244            },
    245            error=response.error if response.HasField("error") else None,
    246        )
    

Run inference using a specified model.

Args: model_uid: The UID of the model to use for inference. inputs: Dictionary
mapping tensor names to tensors.

Returns: ForwardResponse containing: outputs: Dictionary mapping tensor names
to output tensors error: Optional error information if inference failed

