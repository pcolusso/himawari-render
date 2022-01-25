//
//  ContentView.swift
//  himawari-macos
//
//  Created by Paul Colusso on 25/1/22.
//

import SwiftUI

struct ContentView: View {
    @Binding public var manager: WallpaperConfig
    
    var body: some View {
        Button("Do the thing") {
            manager.generate_wallpaper()
        }
    }
}
