//
//  ViewController.swift
//  macos-frontend
//
//  Created by Paul Colusso on 18/5/19.
//  Copyright © 2019 Paul Colusso. All rights reserved.
//

import Cocoa

class ViewController: NSViewController {

    override func viewDidLoad() {
        super.viewDidLoad()

        // Do any additional setup after loading the view.
    }
    @IBAction func doTheThing(_ sender: Any) {
        just_do_it();
    }
    
    override var representedObject: Any? {
        didSet {
        // Update the view, if already loaded.
        }
    }


}

