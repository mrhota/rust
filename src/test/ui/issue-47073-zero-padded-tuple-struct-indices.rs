// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

type Guilty = bool;
type FineDollars = u32;

struct Verdict(Guilty, Option<FineDollars>);

fn main() {
    let justice = Verdict(true, Some(2718));
    let _condemned = justice.00;
    //~^ ERROR invalid tuple or struct index
    let _punishment = justice.001;
    //~^ ERROR invalid tuple or struct index
}
