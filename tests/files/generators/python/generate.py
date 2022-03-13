#!/usr/bin/env python3

from simple_pb2 import SuperSimple, Message
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