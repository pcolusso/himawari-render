//
//  Wallpaper.swift
//  himawari-macos
//
//  Created by Paul Colusso on 25/1/22.
//

import Cocoa
import Network

class WallpaperConfig : ObservableObject {
    let baseUrl = FileManager.default.urls(for: .cachesDirectory, in: .userDomainMask).first!
    var filename: String
    @Published var width: UInt32
    @Published var height: UInt32
    var wallpaperExists: Bool
    var quality: UInt32
    var connected = false
    
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
        
        // Need to also subscribe to updates when the network comes offline/online.
        let monitor = NWPathMonitor()
        monitor.pathUpdateHandler = { pathUpdateHandler in
            if pathUpdateHandler.status == .satisfied {
                print("Internet connection online.")
                self.connected = true
            } else {
                print("There's no internet connection, halting updates")
                self.connected = false
            }
        }
        
        let queue = DispatchQueue(label: "InternetConnectionMonitor")
        monitor.start(queue: queue)
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
            let width = (screen.frame.width * screen.backingScaleFactor).rounded()
            let height = (screen.frame.height * screen.backingScaleFactor).rounded()
            let resolution = Resolution(width: UInt32(width), height: UInt32(height))
            
            resolutions.insert(resolution)
        }
        
        return Array(resolutions)
    }
    
    func clear_files() {
        let files = try! FileManager.default.contentsOfDirectory(at: baseUrl, includingPropertiesForKeys: nil, options: .skipsHiddenFiles)
        
        for f in files where f.pathExtension == "jpg" {
            try! FileManager.default.removeItem(at: f)
        }
    }
    
    @objc func generate_wallpaper() {
        while(!connected) {
            print("No connection, delaying refresh")
            sleep(10)
        }
        print("Generating wallpaper...")
        let fileManager = FileManager.default
        let oldPath = url()
        // By using a new filename, I'm hoping that macOS will recognise a new wallpaper and actually update it.
        let newFile = UUID().uuidString + ".jpg" // Create a new filename
        let newPath = baseUrl.appendingPathComponent(newFile) // Create a new path to pass
        let resolutions = discover_resolutions()
        
        let res = single_wallpaper_file(resolutions[0].width, resolutions[0].height, quality, newPath.path)
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
