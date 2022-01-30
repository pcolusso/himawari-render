//
//  Wallpaper.swift
//  himawari-macos
//
//  Created by Paul Colusso on 25/1/22.
//

import Cocoa

class WallpaperConfig : ObservableObject {
    // TODO: Use a library folder instead.
    let baseUrl = FileManager.default.homeDirectoryForCurrentUser
        .appendingPathComponent("Downloads")
    var filename: String
    @Published var width: UInt32
    @Published var height: UInt32
    var wallpaperExists: Bool
    var quality: UInt32
    
    init() {
        filename = WallpaperConfig.newFilename()
        width = 3840
        height = 2160
        wallpaperExists = false
        quality = 2
    }
    
    func enroll() {
        // Hook whenever space is changed, non-visible spaces are not updated so let's update it when switching to it.
        NSWorkspace.shared.notificationCenter.addObserver(self, selector: #selector(self.set_wallpaper), name: NSWorkspace.activeSpaceDidChangeNotification, object: nil)
        // Hook when device wakes from sleep, as timers don't fire when asleep, so it's likely out of date.
        NSWorkspace.shared.notificationCenter.addObserver(self, selector: #selector(self.set_wallpaper), name: NSWorkspace.didWakeNotification, object: nil)
        // Set up a timer to fire every 10 minites.
        Timer.scheduledTimer(timeInterval: 600.0, target: self, selector: #selector(self.generate_wallpaper), userInfo: nil, repeats: true)
        print("Enrollment made")
    }
    
    func url() -> URL {
        return baseUrl.appendingPathComponent(filename)
    }
    
    static func newFilename() -> String {
        return UUID().uuidString + ".jpg"
    }
    
    struct Resolution: Hashable {
        let width: UInt32
        let height: UInt32
    }
    
    func discover_resolutions() -> [Resolution] {
        let screens = NSScreen.screens
        var resolutions = Set<Resolution>()
        
        for screen in screens {
            let width = screen.frame.width.rounded()
            let height = screen.frame.height.rounded()
            let resolution = Resolution(width: UInt32(width), height: UInt32(height))
            
            resolutions.insert(resolution)
        }
        
        return Array(resolutions)
    }
    
    @objc func generate_wallpaper() {
        print("Generating wallpaper...")
        let fileManager = FileManager.default
        let oldPath = url()
        // By using a new filename, I'm hoping that macOS will recognise a new wallpaper and actually update it.
        let newFile = UUID().uuidString + ".jpg" // Create a new filename
        let newPath = baseUrl.appendingPathComponent(newFile) // Create a new path to pass
        let resolutions = discover_resolutions()
        
        let res = single_wallpaper_file(width, height, quality, newPath.path)
        let resStr = String(cString: res!)
        print("Res: \(resStr)")
        
        // If we're successful...
        if (resStr == "Success") {
            print("Image saved to Downloads")
            if (fileManager.fileExists(atPath: oldPath.path)) {
                try! fileManager.removeItem(at: oldPath) // Cleanup old file.
            }
            self.wallpaperExists = true // Set the flag the file now does exist.
            filename = newFile // Set our new filename, so we can set it.
            self.set_wallpaper()
            
        } else {
            // TODO: Error handling
            print("Error generating image")
        }
        
        res!.deallocate()
    }
    
    @objc func set_wallpaper() {
        if !self.wallpaperExists {
            print("Wallpaper hasn't been made yet, skipping.")
            return
        }
        
        print("Setting wallpaper to \(filename)")
        
        let screens = NSScreen.screens
        
        for screen in screens {
            try! NSWorkspace.shared.setDesktopImageURL(url(), for: screen, options: [:])
        }
    }
}
