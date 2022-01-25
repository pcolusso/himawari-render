//
//  himawari_macosApp.swift
//  himawari-macos
//
//  Created by Paul Colusso on 25/1/22.
//

import SwiftUI

@main
struct himawari_macosApp: App {
    @State public var manager = WallpaperConfig()
    
    init() {
        manager.enroll()
    }
    
    var body: some Scene {
        WindowGroup {
            ContentView(manager: $manager)
        }
    }
}
