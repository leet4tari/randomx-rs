// Copyright 2019. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::env;

use cmake::Config;

#[allow(clippy::too_many_lines)]
fn main() {
    // Get the target triple (e.g., x86_64-apple-ios, x86_64-linux-gnu, etc.)
    let target = env::var("CARGO_CFG_TARGET").unwrap_or_else(|_| "unknown".to_string());

    // Extract the architecture part of the target triple (e.g., "x86_64" from "x86_64-apple-ios")
    let target_arch = target.split('-').next().unwrap_or("unknown");

    // Determine if we are cross-compiling
    let host_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "unknown".to_string());
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "linux".to_string());
    let is_cross_compiling = host_os != target_os;

    // Configure RandomX and set CMake options
    let mut config = Config::new(env::var("RANDOMX_DIR").unwrap_or_else(|_| "RandomX".to_string()));

    if is_cross_compiling {
        config.define("CMAKE_CROSSCOMPILING", "TRUE");
        // Set DARCH to the CPU architecture (extracted from the target triple)
        config.define("DARCH", target_arch);
    } else {
        config.define("DARCH", "native");
    }

    // Build the RandomX configuration
    let randomx_path = config.build();

    // Output linking information
    println!("cargo:rustc-link-search=native={}/lib", randomx_path.display());
    println!("cargo:rustc-link-lib=static=randomx");

    //let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or("linux".to_string());

    // Determine the appropriate C++ library to link against based on the target OS
    let dylib_name = match target_os.as_str() {
        "macos" | "ios" => "c++",  // macOS and iOS use "c++"
        "windows" => "msvcrt",     // Use MSVC runtime on Windows
        _ => "stdc++",             // Default for other systems (Linux, etc.)
    };

    // Output the dylib linking information
    println!("cargo:rustc-link-lib=dylib={}", dylib_name);
}
