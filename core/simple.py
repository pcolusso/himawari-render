from appscript import app, mactypes
import ctypes, platform, time

lib_names = {
    "Windows": "himawari_render.dll",
    "Darwin": "libhimawari_render.dylib"
}
lib = ctypes.cdll.LoadLibrary('target/debug/' + lib_names.get(platform.system()))
lib.wallpaper_pls.argtypes = (ctypes.c_char_p, ctypes.c_uint, ctypes.c_uint)

path = ""
fname = path + 'latest_render.png'

while True:
  lib.wallpaper_pls(fname.encode(), 1920, 1200)
  app('Finder').desktop_picture.set(mactypes.File(fname))
  time.sleep(10 * 60)