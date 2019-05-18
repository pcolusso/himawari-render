import ctypes

# This will need to be altered per-platform
lib = ctypes.cdll.LoadLibrary('target/release/libhimawari_render.dylib')

lib.save_planet.restype = ctypes.c_char_p
lib.save_planet.argtypes = (ctypes.c_char_p, ctypes.c_uint)

result = lib.save_planet(b'py.png', 8)

print(result.value)