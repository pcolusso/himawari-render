//
//  Wallpaper.swift
//  himawari-macos
//
//  Created by Paul Colusso on 25/1/22.
//

import Cocoa

class WallpaperConfig {
    let url = FileManager.default.homeDirectoryForCurrentUser
        .appendingPathComponent("Downloads")
        .appendingPathComponent("wallpaper.jpg")
    var wallpaperExists = false
    
    func enroll() {
        NSWorkspace.shared.notificationCenter.addObserver(self, selector: #selector(self.set_wallpaper), name: NSWorkspace.activeSpaceDidChangeNotification, object: nil)
        print("Enrollment made")
        Timer.scheduledTimer(timeInterval: 600.0, target: self, selector: #selector(self.generate_wallpaper), userInfo: nil, repeats: true)
    }
    
    @objc func generate_wallpaper() {
        print("Generating wallpaper...")
        DispatchQueue.global(qos: .background).async {
            let res = single_wallpaper_file(3840, 2160, 2, self.url.path)
            print("Res: \(String(cString: res!))")
            res!.deallocate()
            
            DispatchQueue.main.async {
                print("Image saved to Downloads")
                self.wallpaperExists = true
                self.set_wallpaper()
            }
        }
    }
    
    @objc func set_wallpaper() {
        if !self.wallpaperExists {
            print("Wallpaper hasn't been made yet, skipping.")
            return
        }
        
        print("Setting wallpaper")
        
        let screens = NSScreen.screens
        
        for screen in screens {
            print("Screen: \(screen.localizedName)")
            try! NSWorkspace.shared.setDesktopImageURL(url, for: screen, options: [:])
        }
    }
}
