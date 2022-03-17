#!/usr/bin/env python3

from simple_pb2 import SuperSimple, Message
from types_pb2 import SimpleTypes, RepeatedTypes
import api.api_pb2 as api
import api.v1.v1_pb2 as api_v1

simplemessage = SuperSimple()
simplemessage.serial_number = "serial"
simplemessage.firmware_version = "firmware"
simplemessage.vendor = "vendor"
simplemessage.product = "product"
with open("../../bin/python.simple.bin", "wb") as fd:
    fd.write(simplemessage.SerializeToString())


m = Message()
m.test = "teststr"
with open("../../bin/python.oneof.simple.bin", "wb") as fd:
    fd.write(m.SerializeToString())


m = Message()
# Python sucks
# AttributeError: Assignment not allowed to field "ss" in protocol message object.
# m.ss = simplemessage
m.ss.serial_number = "serial"
m.ss.firmware_version = "firmware"
m.ss.vendor = "vendor"
m.ss.product = "product"
m.something_else = "something else"
with open("../../bin/python.oneof.embedded.bin", "wb") as fd:
    fd.write(m.SerializeToString())


m = api.Message()
m.v1_request.getInfo.SetInParent()
with open("../../bin/python.api.getInfo.bin", "wb") as fd:
    fd.write(m.SerializeToString())

m = SimpleTypes()
m.int32 = -69
m.int64 = -9223372036854775808
m.uint32 = 42
m.uint64 = 1
m.sint32 = -69
m.sint64 = 69
m.fixed32 = 2**32-1
m.fixed64 = 42
m.sfixed32 = 2**31-1
m.sfixed64 = 42
m.double = 1
m.float = 3.1415926535
m.bool = 1
m.string = "🐉"
m.bytes = b"ASDF"
with open("../../bin/python.types.simple.bin", "wb") as fd:
    fd.write(m.SerializeToString())

m = RepeatedTypes()
# m.int32.extend([-69, 69])
m.int32.extend([4,300])
# m.int64.extend([69, -69])
# m.uint32.extend([42, 420])
# m.uint64.extend([42, 420])
# m.sint32.extend([-69, 69])
# m.sint64.extend([69, -69])
# m.fixed32.extend([2**32-1, 1])
# m.fixed64.extend([42, 2**64-1])
# m.sfixed32.extend([2**31-1, -69])
# m.sfixed64.extend([42, -42])
# m.double.extend([1, 3.1415926535])
# m.float.extend([3.1415926535, 1])
# m.bool.extend([1, 0])
# m.string.extend(["🐉", "अरे"])
m.string.extend(["la", "la"])
# m.bytes.extend([b"ABCD"])
# m.int32_notpacked.extend([-69, 69])
m.int32_notpacked.extend([4,300])
with open("../../bin/python.types.repeated.bin", "wb") as fd:
    fd.write(m.SerializeToString())