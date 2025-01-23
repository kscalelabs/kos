# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc
import warnings

from google.protobuf import empty_pb2 as google_dot_protobuf_dot_empty__pb2
from kos import process_manager_pb2 as kos_dot_process__manager__pb2

GRPC_GENERATED_VERSION = '1.68.1'
GRPC_VERSION = grpc.__version__
_version_not_supported = False

try:
    from grpc._utilities import first_version_is_lower
    _version_not_supported = first_version_is_lower(GRPC_VERSION, GRPC_GENERATED_VERSION)
except ImportError:
    _version_not_supported = True

if _version_not_supported:
    raise RuntimeError(
        f'The grpc package installed is at version {GRPC_VERSION},'
        + f' but the generated code in kos/process_manager_pb2_grpc.py depends on'
        + f' grpcio>={GRPC_GENERATED_VERSION}.'
        + f' Please upgrade your grpc module to grpcio>={GRPC_GENERATED_VERSION}'
        + f' or downgrade your generated code using grpcio-tools<={GRPC_VERSION}.'
    )


class ProcessManagerServiceStub(object):
    """The ProcessManagerService manages processes like video streaming.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.StartKClip = channel.unary_unary(
                '/kos.processmanager.ProcessManagerService/StartKClip',
                request_serializer=kos_dot_process__manager__pb2.KClipStartRequest.SerializeToString,
                response_deserializer=kos_dot_process__manager__pb2.KClipStartResponse.FromString,
                _registered_method=True)
        self.StopKClip = channel.unary_unary(
                '/kos.processmanager.ProcessManagerService/StopKClip',
                request_serializer=google_dot_protobuf_dot_empty__pb2.Empty.SerializeToString,
                response_deserializer=kos_dot_process__manager__pb2.KClipStopResponse.FromString,
                _registered_method=True)


class ProcessManagerServiceServicer(object):
    """The ProcessManagerService manages processes like video streaming.
    """

    def StartKClip(self, request, context):
        """Starts kclip recording.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def StopKClip(self, request, context):
        """Stops kclip recording.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_ProcessManagerServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'StartKClip': grpc.unary_unary_rpc_method_handler(
                    servicer.StartKClip,
                    request_deserializer=kos_dot_process__manager__pb2.KClipStartRequest.FromString,
                    response_serializer=kos_dot_process__manager__pb2.KClipStartResponse.SerializeToString,
            ),
            'StopKClip': grpc.unary_unary_rpc_method_handler(
                    servicer.StopKClip,
                    request_deserializer=google_dot_protobuf_dot_empty__pb2.Empty.FromString,
                    response_serializer=kos_dot_process__manager__pb2.KClipStopResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'kos.processmanager.ProcessManagerService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))
    server.add_registered_method_handlers('kos.processmanager.ProcessManagerService', rpc_method_handlers)


 # This class is part of an EXPERIMENTAL API.
class ProcessManagerService(object):
    """The ProcessManagerService manages processes like video streaming.
    """

    @staticmethod
    def StartKClip(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(
            request,
            target,
            '/kos.processmanager.ProcessManagerService/StartKClip',
            kos_dot_process__manager__pb2.KClipStartRequest.SerializeToString,
            kos_dot_process__manager__pb2.KClipStartResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)

    @staticmethod
    def StopKClip(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(
            request,
            target,
            '/kos.processmanager.ProcessManagerService/StopKClip',
            google_dot_protobuf_dot_empty__pb2.Empty.SerializeToString,
            kos_dot_process__manager__pb2.KClipStopResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)
