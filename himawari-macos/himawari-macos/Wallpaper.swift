//
//  Wallpaper.swift
//  himawari-macos
//
//  Created by Paul Colusso on 25/1/22.
//

import Cocoa

func set_wallpaper() {
    let path = "/Users/paulcolusso/Downloads/wallpaper.jpg"
    single_wallpaper_file(3840, 2160, 2, path)
    print("Image saved to Downloads")

    let screens = NSScreen.screens
    let url = URL(fileURLWithPath: path)
    
    for screen in screens {
        try! NSWorkspace.shared.setDesktopImageURL(url, for: screen, options: [:])
    }
    
    // https://apple.stackexchange.com/questions/433794/seamlessly-change-all-desktop-spaces-wallpaper-on-mac-without-killall-dock
    // https://developer.apple.com/forums/thread/98722 ?
    
}
