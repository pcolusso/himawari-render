//
//  ContentView.swift
//  himawari-macos
//
//  Created by Paul Colusso on 25/1/22.
//

import SwiftUI
import Combine

struct ContentView: View {
    @EnvironmentObject var manager: WallpaperConfig
    @State var quality = "2"
    @State var width = "3840"
    @State var height = "2160"
    
    var body: some View {
        VStack(alignment: .center, spacing: 10.0){
            Form {
                TextField("Width of the generated wallpaper: ", text: $width)
                TextField("Height of the generated wallpaper: ", text: Binding<String>(
                    get: {
                        "poop"
                    },
                    set: { newValue in
                        
                    }
                ))
                    .onReceive(Just(width)) { newValue in
                        let filtered = newValue.filter { "0123456789".contains($0) }
                        if filtered != newValue {
                            self.quality = filtered
                        }
                    }
                TextField("Quality Level: ", text: $quality)
                    .onReceive(Just(quality)) { newValue in
                        let filtered = newValue.filter { "1234".contains($0) }
                        if filtered != newValue {
                            self.quality = filtered
                        }
                    }
                Button("Manually Refresh Now") {
                    DispatchQueue.global(qos: .background).async {
                        manager.generate_wallpaper()
                    }
                }
            }
        }.frame(width: 600, height: 160, alignment: .center)
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
