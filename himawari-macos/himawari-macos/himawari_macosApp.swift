//
//  himawari_macosApp.swift
//  himawari-macos
//
//  Created by Paul Colusso on 25/1/22.
//

import SwiftUI

@main
struct himawari_macosApp: App {
    @StateObject var manager = WallpaperConfig()
    
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(manager)
                .onAppear() {
                    manager.enroll()
                }
        }
    }
}
