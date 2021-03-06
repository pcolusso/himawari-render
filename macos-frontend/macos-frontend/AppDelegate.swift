//
//  AppDelegate.swift
//  macos-frontend
//
//  Created by Paul Colusso on 18/5/19.
//  Copyright © 2019 Paul Colusso. All rights reserved.
//

import Cocoa

@NSApplicationMain
class AppDelegate: NSObject, NSApplicationDelegate {

    let statusItem = NSStatusBar.system.statusItem(withLength:NSStatusItem.squareLength)
    let timer = Timer.scheduledTimer(timeInterval: 600, target: self, selector: #selector(AppDelegate.refreshNow(_:)), userInfo: nil, repeats: true)


    func applicationDidFinishLaunching(_ aNotification: Notification) {
        // Insert code here to initialize your application
        
        if let button = statusItem.button {
            button.image = NSImage(named:NSImage.Name("globe"))
        }
        
        constructMenu()
    }

    func applicationWillTerminate(_ aNotification: Notification) {
        // Insert code here to tear down your application
    }
    
    func createAppDataDirectory() -> URL {
        let applicationSupportPath = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first
        let dataPath = applicationSupportPath!.appendingPathComponent("Himawari")
        try! FileManager.default.createDirectory(atPath: dataPath.path, withIntermediateDirectories: true, attributes: nil)
        
        return dataPath
    }
    
    func setWallpaper(path: URL) {
        let sharedWorkspace = NSWorkspace.shared
        let screens = NSScreen.screens
        let options: NSDictionary = [
            "NSWorkspaceDesktopImageFillColorKey": NSColor.black,
            //"NSWorkspaceDesktopImageScalingKey":, "",
        ]
        
        for screen in screens{
            try! sharedWorkspace.setDesktopImageURL(path, for: screen, options: options as! [NSWorkspace.DesktopImageOptionKey : Any])
        }
    }

    @objc func refreshNow(_ sender: Any?) {
        let targetPath = createAppDataDirectory().appendingPathComponent("latest_render.png")
        let firstScreen = NSScreen.screens.first!
        let path = NSString(string: targetPath.path).utf8String
        let pathBuf = UnsafeMutablePointer<Int8>(mutating: path)
        
        // Appears macOS gets the simulated res for Retina displays...
        wallpaper_pls(pathBuf, UInt32(firstScreen.visibleFrame.width), UInt32(firstScreen.visibleFrame.height))
        setWallpaper(path: targetPath)
    }
    
    func constructMenu() {
        let menu = NSMenu()
        
        menu.addItem(NSMenuItem(title: "Refresh Now", action: #selector(AppDelegate.refreshNow(_:)), keyEquivalent: "P"))
        menu.addItem(NSMenuItem(title: "Quit", action: #selector(NSApplication.terminate(_:)), keyEquivalent: "q"))
        
        statusItem.menu = menu
    }
}

