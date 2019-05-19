import ctypes, platform

lib_names = {
    "Windows": "himawari_render.dll",
    "macOS": "libhimawari_render.dylib"
}
lib = ctypes.cdll.LoadLibrary('target/release/' + lib_names.get(platform.system()))

lib.save_planet.restype = ctypes.c_char_p
lib.save_planet.argtypes = (ctypes.c_char_p, ctypes.c_uint)

result = lib.save_planet(b'py.png', 8)

print(result)