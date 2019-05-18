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

    @objc func refreshNow(_ sender: Any?) {
        // TODO: Appears the file does not save. Restriction? Limitation of debug env?
        let path = NSString("~/from-swift.png").utf8String
        let pathBuf = UnsafeMutablePointer<Int8>(mutating: path)
        let result = save_planet(pathBuf, 4)
        
        if let returnCode = result {
            let asString = String(cString: returnCode)
            print(asString) // Result is invalid, so is the response from Python. Perhaps lib is free'ing the pointer before we can use it.
        }
    }
    
    func constructMenu() {
        let menu = NSMenu()
        
        menu.addItem(NSMenuItem(title: "Refresh Now", action: #selector(AppDelegate.refreshNow(_:)), keyEquivalent: "P"))
        menu.addItem(NSMenuItem(title: "Quit", action: #selector(NSApplication.terminate(_:)), keyEquivalent: "q"))
        
        statusItem.menu = menu
    }
}

