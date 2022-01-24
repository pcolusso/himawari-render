//
//  ContentView.swift
//  himawari-macos
//
//  Created by Paul Colusso on 25/1/22.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        Button("Do the thing") {
            set_wallpaper()
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
