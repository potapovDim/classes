from ctypes import cdll

lib = cdll.LoadLibrary('target/release/libclass_3.dylib')
lib.process()

print("Done")