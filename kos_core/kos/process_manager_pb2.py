# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: kos/process_manager.proto
# Protobuf Python Version: 5.28.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import runtime_version as _runtime_version
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
_runtime_version.ValidateProtobufRuntimeVersion(
    _runtime_version.Domain.PUBLIC,
    5,
    28,
    1,
    '',
    'kos/process_manager.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.protobuf import empty_pb2 as google_dot_protobuf_dot_empty__pb2
from kos import common_pb2 as kos_dot_common__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x19kos/process_manager.proto\x12\x12kos.processmanager\x1a\x1bgoogle/protobuf/empty.proto\x1a\x10kos/common.proto\"#\n\x11KClipStartRequest\x12\x0e\n\x06\x61\x63tion\x18\x01 \x01(\t\"\\\n\x12KClipStartResponse\x12\x16\n\tclip_uuid\x18\x01 \x01(\tH\x00\x88\x01\x01\x12 \n\x05\x65rror\x18\x02 \x01(\x0b\x32\x11.kos.common.ErrorB\x0c\n\n_clip_uuid\"[\n\x11KClipStopResponse\x12\x16\n\tclip_uuid\x18\x01 \x01(\tH\x00\x88\x01\x01\x12 \n\x05\x65rror\x18\x02 \x01(\x0b\x32\x11.kos.common.ErrorB\x0c\n\n_clip_uuid2\xc0\x01\n\x15ProcessManagerService\x12[\n\nStartKClip\x12%.kos.processmanager.KClipStartRequest\x1a&.kos.processmanager.KClipStartResponse\x12J\n\tStopKClip\x12\x16.google.protobuf.Empty\x1a%.kos.processmanager.KClipStopResponseBP\n\x16\x63om.kos.processmanagerZ!kos/processmanager;processmanager\xaa\x02\x12KOS.ProcessManagerb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'kos.process_manager_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\026com.kos.processmanagerZ!kos/processmanager;processmanager\252\002\022KOS.ProcessManager'
  _globals['_KCLIPSTARTREQUEST']._serialized_start=96
  _globals['_KCLIPSTARTREQUEST']._serialized_end=131
  _globals['_KCLIPSTARTRESPONSE']._serialized_start=133
  _globals['_KCLIPSTARTRESPONSE']._serialized_end=225
  _globals['_KCLIPSTOPRESPONSE']._serialized_start=227
  _globals['_KCLIPSTOPRESPONSE']._serialized_end=318
  _globals['_PROCESSMANAGERSERVICE']._serialized_start=321
  _globals['_PROCESSMANAGERSERVICE']._serialized_end=513
# @@protoc_insertion_point(module_scope)
