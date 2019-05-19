import ctypes, platform

lib_names = {
    "Windows": "himawari_render.dll",
    "Darwin": "libhimawari_render.dylib"
}
lib = ctypes.cdll.LoadLibrary('target/debug/' + lib_names.get(platform.system()))

# lib.save_planet.restype = ctypes.c_char_p
# lib.save_planet.argtypes = (ctypes.c_char_p, ctypes.c_uint)

# result = lib.save_planet(b'py.png', 8)

# print(result)

lib.wallpaper_pls.argtypes = (ctypes.c_char_p, ctypes.c_uint, ctypes.c_uint)
lib.wallpaper_pls(b"here.png", 1920, 1200)