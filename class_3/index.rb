require 'ffi'

module Hello
  extend FFI::Library
  ffi_lib 'target/release/libclass_3.dylib'
  attach_function :process, [], :void
end

Hello.process

puts 'Done'