"""LED Matrix service client."""

from typing import NotRequired, TypedDict

import grpc
from google.protobuf.empty_pb2 import Empty

from kos_protos import common_pb2, led_matrix_pb2, led_matrix_pb2_grpc


class MatrixInfo(TypedDict):
    """Information about the LED matrix.

    Args:
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


class LEDMatrixServiceClient:
    """Client for the LEDMatrixService.

    This service allows controlling an LED matrix display.
    """

    def __init__(self, channel: grpc.Channel) -> None:
        """Initialize the LED matrix service client.

        Args:
            channel: gRPC channel to use for communication.
        """
        self.stub = led_matrix_pb2_grpc.LEDMatrixServiceStub(channel)

    def get_matrix_info(self) -> MatrixInfo:
        """Get information about the LED matrix including dimensions and capabilities.

        Returns:
            MatrixInfo containing:
                width: Width in pixels
                height: Height in pixels
                brightness_levels: Number of brightness levels supported
                color_capable: Whether the matrix supports color
                bits_per_pixel: Number of bits used to represent each pixel
                error: Optional error information
        """
        response = self.stub.GetMatrixInfo(Empty())
        return MatrixInfo(
            width=response.width,
            height=response.height,
            brightness_levels=response.brightness_levels,
            color_capable=response.color_capable,
            bits_per_pixel=response.bits_per_pixel,
            error=response.error if response.HasField("error") else None,
        )

    def write_buffer(self, buffer: bytes) -> common_pb2.ActionResponse:
        """Write binary on/off states to the LED matrix.

        The buffer should be width * height / 8 bytes long, where each bit
        represents one LED's on/off state.

        Args:
            buffer: Binary buffer containing LED states

        Returns:
            ActionResponse indicating success/failure of the write operation.
        """
        request = led_matrix_pb2.WriteBufferRequest(buffer=buffer)
        return self.stub.WriteBuffer(request)

    def write_color_buffer(self, buffer: bytes, brightness: int = 255) -> common_pb2.ActionResponse:
        """Write RGB color data to the LED matrix.

        The buffer should be width * height * 3 bytes long, where each pixel
        is represented by three consecutive bytes for R, G, and B values.

        Args:
            buffer: RGB buffer containing color data
            brightness: Global brightness level (0-255)

        Returns:
            ActionResponse indicating success/failure of the write operation.
        """
        request = led_matrix_pb2.WriteColorBufferRequest(buffer=buffer, brightness=brightness)
        return self.stub.WriteColorBuffer(request)
