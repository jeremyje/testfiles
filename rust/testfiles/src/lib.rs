// Copyright 2023 Jeremy Edwards
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use include_dir::{include_dir, Dir};

static TESTFILES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../testfiles/");

pub fn get() -> &'static Dir<'static> {
    &TESTFILES_DIR
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_with_glob() {
        let result = get();
        let files = result.find("document/*").unwrap();
        let col: Vec<_> = files.collect();
        
        assert_eq!(col.len(), 8);
    }
}
