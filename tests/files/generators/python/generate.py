#!/usr/bin/env python3

from simple_pb2 import SuperSimple, Message

simplemessage = SuperSimple()
simplemessage.serial_number = "serial"
simplemessage.firmware_version = "firmware"
simplemessage.vendor = "vendor"
simplemessage.product = "product"

with open("../../bin/python.simple.bin", "wb") as fd:
    fd.write(simplemessage.SerializeToString())


m = Message()
# Python sucks
# AttributeError: Assignment not allowed to field "ss" in protocol message object.
# m.ss = simplemessage
m.ss.serial_number = "serial"
m.ss.firmware_version = "firmware"
m.ss.vendor = "vendor"
m.ss.product = "product"
m.test = "teststr"

with open("../../bin/python.embedded.simple.bin", "wb") as fd:
    fd.write(m.SerializeToString())
