// Copyright 2018 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.


extern crate uuid;


use uuid::Uuid;


fn main() {
    println!("{}", Uuid::new_v4());
}
