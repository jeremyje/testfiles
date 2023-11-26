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

//static TESTFILES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../../testfiles/");
//static TESTFILES_DIR: Dir<'_> = include_dir!("/home/coder/project/testfiles/testfiles/");
//static DOCUMENT_A: &'static [u8] = include_bytes!("/home/coder/project/testfiles/testfiles/document/a");
//static SAMPLE_VIDEO_MP4: &'static [u8] = include_bytes!("/home/coder/project/testfiles/testfiles/video/sample-from-clipchamp-480p.mp4");
static TESTFILES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/testfiles/");
static DOCUMENT_A: &'static [u8] = include_bytes!("testfiles/document/a");
static SAMPLE_VIDEO_MP4: &'static [u8] = include_bytes!("testfiles/video/sample-from-clipchamp-480p.mp4");


/// get returns a Dir (virtual directory) that contains all the test files.
pub fn get() -> &'static Dir<'static> {
    &TESTFILES_DIR
}

/// a returns an example text file with the contents 'a'.
pub fn a() -> &'static [u8] {
    &DOCUMENT_A
}

/// sample_video_mp4 returns an example MP4 with 720p resolution.
pub fn sample_video_mp4() -> &'static [u8] {
    &SAMPLE_VIDEO_MP4
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

    #[test]
    fn get_a_file() {
        let bytes = a();
        assert_eq!(bytes, b"a");
    }

    #[test]
    fn get_sample_video_mp4() {
        let bytes = sample_video_mp4();
        assert_eq!(bytes.len(), 522767);
    }
}
