"""LED Matrix service client."""

from typing import NotRequired, TypedDict, Unpack

import grpc
from google.protobuf.empty_pb2 import Empty

from kos_protos import common_pb2, led_matrix_pb2_grpc
from kos_protos.led_matrix_pb2 import (
    GetMatrixInfoResponse,
    WriteBufferRequest,
    WriteColorBufferRequest,
)


class MatrixInfo(TypedDict):
    """TypedDict containing LED matrix configuration details.

    A dictionary type describing the physical layout and capabilities
    of the LED matrix display.

    Fields:
        width: Width in pixels
        height: Height in pixels
        brightness_levels: Number of brightness levels supported (1 for binary on/off)
        color_capable: Whether the matrix supports color
        bits_per_pixel: Number of bits used to represent each pixel
        error: Optional error information
    """

    width: int
    height: int
    brightness_levels: int
    color_capable: bool
    bits_per_pixel: int
    error: NotRequired[common_pb2.Error | None]


class ImageData(TypedDict):
    """Image data to be written to the LED matrix.

    Fields:
        buffer: Raw image data bytes
        width: Image width in pixels
        height: Image height in pixels
        format: Pixel format specification (e.g. 'RGB888', 'BGR888', 'RGB565', 'MONO8')
        brightness: Global brightness level (0-255)
    """

    buffer: bytes
    width: int
    height: int
    format: str
    brightness: int


class ActionResponse(TypedDict):
    """Response indicating success/failure of an action."""

    success: bool
    error: NotRequired[common_pb2.Error | None]


class LEDMatrixServiceClient:
    """Client for the LED matrix service.

    This client provides methods to interact with an LED matrix display,
    including querying its capabilities and writing image data to it.
    """

    def __init__(self, channel: grpc.Channel) -> None:
        """Initialize the LED matrix service client.

        Args:
            channel: gRPC channel for communication with the service
        """
        self.stub = led_matrix_pb2_grpc.LEDMatrixServiceStub(channel)

    def get_matrix_info(self) -> GetMatrixInfoResponse:
        """Get information about the LED matrix display.

        Returns:
            GetMatrixInfoResponse containing:
            - width: Width in pixels
            - height: Height in pixels
            - brightness_levels: Number of brightness levels supported
            - color_capable: Whether the matrix supports color
            - bits_per_pixel: Number of bits used to represent each pixel
            - error: Optional error information
        """
        return self.stub.GetMatrixInfo(Empty())

    def write_buffer(self, buffer: bytes) -> common_pb2.ActionResponse:
        """Write raw buffer data to the LED matrix.

        This method is for writing pre-formatted data that matches the matrix's
        native format. For writing color images, use write_color_buffer instead.

        Args:
            buffer: Raw buffer data bytes in the matrix's native format

        Returns:
            ActionResponse indicating if the write was successful
        """
        request = WriteBufferRequest(buffer=buffer)
        return self.stub.WriteBuffer(request)

    def write_color_buffer(self, **kwargs: Unpack[ImageData]) -> common_pb2.ActionResponse:
        """Write a color image buffer to the LED matrix.

        Args:
            **kwargs: Image data containing:
                     buffer: Raw image data bytes
                     width: Image width in pixels
                     height: Image height in pixels
                     format: Pixel format specification (e.g. 'RGB888', 'BGR888', 'RGB565', 'MONO8')
                     brightness: Global brightness level (0-255)

        Returns:
            ActionResponse indicating if the write was successful
        """
        request = WriteColorBufferRequest(**kwargs)
        return self.stub.WriteColorBuffer(request)
