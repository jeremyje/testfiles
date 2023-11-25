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

package testfiles

import (
	"io"
	"reflect"
	"testing"
)

func TestGet(t *testing.T) {
	tests := []struct {
		path string
		want []byte
	}{
		{
			path: "testfiles/document/empty",
			want: []byte{},
		},
		{
			path: "testfiles/document/a",
			want: []byte{'a'},
		},
	}

	for _, tc := range tests {
		tc := tc
		t.Run(tc.path, func(t *testing.T) {
			f, err := Get().Open(tc.path)
			if err != nil {
				t.Fatalf("cannot open %s", err)
			}

			got, err := io.ReadAll(f)
			if err != nil {
				t.Fatalf("cannot open %s", err)
			}

			if !reflect.DeepEqual(tc.want, got) {
				t.Fatalf("want: %v, got: %v", tc.want, got)
			}
		})
	}
}
