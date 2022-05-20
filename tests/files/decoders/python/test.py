#!/usr/bin/env python3

from simple_pb2 import SuperSimple, Message
from types_pb2 import SimpleTypes, RepeatedTypes
import api.api_pb2 as api
import api.v1.v1_pb2 as api_v1

def assertEq(actual, expected):
    if expected != actual:
        print(f"Expected value '{expected}' does not match actual value '{actual}'")
        raise

with open("../../bin/twpb.simple.bin", "rb") as fd:
    message = SuperSimple().FromString(fd.read())
    assertEq(message.serial_number, 'serial')
    assertEq(message.firmware_version, 'firmware')
    assertEq(message.vendor, 'vendor')
    assertEq(message.product, 'product')

with open("../../bin/twpb.oneof.simple.bin", "rb") as fd:
    message = Message().FromString(fd.read())
    assertEq(message.test, 'teststr')
    assertEq(message.something_else, '')

with open("../../bin/twpb.oneof.embedded.bin", "rb") as fd:
    message = Message().FromString(fd.read())
    assertEq(message.ss.serial_number, 'serial')
    assertEq(message.ss.firmware_version, 'firmware')
    assertEq(message.ss.vendor, 'vendor')
    assertEq(message.ss.product, 'product')
    assertEq(message.something_else, 'something else')

with open("../../bin/twpb.api.getInfo.bin", "rb") as fd:
    message = api.Message().FromString(fd.read())
    assert hasattr(message.v1_request, 'getInfo')
    assert not hasattr(message.v1_request, 'GetOtherThing')

with open("../../bin/twpb.types.simple.bin", "rb") as fd:
    message = SimpleTypes().FromString(fd.read())
    assertEq(message.int32, -69)
    assertEq(message.int64, -9223372036854775808)
    assertEq(message.uint32, 42)
    assertEq(message.uint64, 1)
    assertEq(message.sint32, -69)
    assertEq(message.sint64, 69)
    assertEq(message.fixed32, 4294967295)
    assertEq(message.fixed64, 42)
    assertEq(message.sfixed32, 2147483647)
    assertEq(message.sfixed64, -9223372036854775807)
    assertEq(message.double, 1.0)
    assertEq(round(message.float, 2), 3.14)
    assertEq(message.bool, True)
    assertEq(message.string, "🐉")
    assertEq(message.bytes, b"ASDF")

with open("../../bin/twpb.types.repeated.bin", "rb") as fd:
    message = RepeatedTypes().FromString(fd.read())
    assertEq(message.int32, [4, -300])
    assertEq(message.int64, [69, -69])
    assertEq(message.uint32, [42, 420])
    assertEq(message.uint64, [42, 420])
    assertEq(message.sint32, [-69, 69])
    assertEq(message.sint64, [69, -69])
    assertEq(message.fixed32, [2**32-1, 1])
    assertEq(message.fixed64, [42, 2**64-1])
    assertEq(message.sfixed32, [2**31-1, -69])
    assertEq(message.sfixed64, [42, -42])
    assertEq(message.double, [1, 3.1415926535])
    # python doesn't read floats in correctly, even the ones it wrote itself
    assertEq(message.float, [3.1415927410125732, 1.0])
    assertEq(message.bool, [1, 0])
    assertEq(message.string, ["🐉", "अरे"])
    assertEq(message.bytes, [b"ASDF", b"ABCD"])
    assertEq(message.int32_notpacked, [4,-300])
