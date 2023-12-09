[1mdiff --git a/Cargo.lock b/Cargo.lock[m
[1mindex 07f2f09..ab6ad52 100644[m
[1m--- a/Cargo.lock[m
[1m+++ b/Cargo.lock[m
[36m@@ -2,6 +2,248 @@[m
 # It is not intended for manual editing.[m
 version = 3[m
 [m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "actix-codec"[m
[32m+[m[32mversion = "0.5.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "617a8268e3537fe1d8c9ead925fca49ef6400927ee7bc26750e90ecee14ce4b8"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "bitflags",[m
[32m+[m[32m "bytes",[m
[32m+[m[32m "futures-core",[m
[32m+[m[32m "futures-sink",[m
[32m+[m[32m "memchr",[m
[32m+[m[32m "pin-project-lite",[m
[32m+[m[32m "tokio",[m
[32m+[m[32m "tokio-util",[m
[32m+[m[32m "tracing",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "actix-files"[m
[32m+[m[32mversion = "0.6.2"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "d832782fac6ca7369a70c9ee9a20554623c5e51c76e190ad151780ebea1cf689"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "actix-http",[m
[32m+[m[32m "actix-service",[m
[32m+[m[32m "actix-utils",[m
[32m+[m[32m "actix-web",[m
[32m+[m[32m "askama_escape",[m
[32m+[m[32m "bitflags",[m
[32m+[m[32m "bytes",[m
[32m+[m[32m "derive_more",[m
[32m+[m[32m "futures-core",[m
[32m+[m[32m "http-range",[m
[32m+[m[32m "log",[m
[32m+[m[32m "mime",[m
[32m+[m[32m "mime_guess",[m
[32m+[m[32m "percent-encoding",[m
[32m+[m[32m "pin-project-lite",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "actix-http"[m
[32m+[m[32mversion = "3.3.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "c2079246596c18b4a33e274ae10c0e50613f4d32a4198e09c7b93771013fed74"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "actix-codec",[m
[32m+[m[32m "actix-rt",[m
[32m+[m[32m "actix-service",[m
[32m+[m[32m "actix-utils",[m
[32m+[m[32m "ahash 0.8.3",[m
[32m+[m[32m "base64",[m
[32m+[m[32m "bitflags",[m
[32m+[m[32m "brotli",[m
[32m+[m[32m "bytes",[m
[32m+[m[32m "bytestring",[m
[32m+[m[32m "derive_more",[m
[32m+[m[32m "encoding_rs",[m
[32m+[m[32m "flate2",[m
[32m+[m[32m "futures-core",[m
[32m+[m[32m "h2",[m
[32m+[m[32m "http",[m
[32m+[m[32m "httparse",[m
[32m+[m[32m "httpdate",[m
[32m+[m[32m "itoa",[m
[32m+[m[32m "language-tags",[m
[32m+[m[32m "local-channel",[m
[32m+[m[32m "mime",[m
[32m+[m[32m "percent-encoding",[m
[32m+[m[32m "pin-project-lite",[m
[32m+[m[32m "rand",[m
[32m+[m[32m "sha1",[m
[32m+[m[32m "smallvec",[m
[32m+[m[32m "tokio",[m
[32m+[m[32m "tokio-util",[m
[32m+[m[32m "tracing",[m
[32m+[m[32m "zstd",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "actix-macros"[m
[32m+[m[32mversion = "0.2.4"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "e01ed3140b2f8d422c68afa1ed2e85d996ea619c988ac834d255db32138655cb"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "quote",[m
[32m+[m[32m "syn 2.0.26",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "actix-router"[m
[32m+[m[32mversion = "0.5.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "d66ff4d247d2b160861fa2866457e85706833527840e4133f8f49aa423a38799"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "bytestring",[m
[32m+[m[32m "http",[m
[32m+[m[32m "regex",[m
[32m+[m[32m "serde",[m
[32m+[m[32m "tracing",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "actix-rt"[m
[32m+[m[32mversion = "2.8.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "15265b6b8e2347670eb363c47fc8c75208b4a4994b27192f345fcbe707804f3e"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "futures-core",[m
[32m+[m[32m "tokio",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "actix-server"[m
[32m+[m[32mversion = "2.2.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "3e8613a75dd50cc45f473cee3c34d59ed677c0f7b44480ce3b8247d7dc519327"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "actix-rt",[m
[32m+[m[32m "actix-service",[m
[32m+[m[32m "actix-utils",[m
[32m+[m[32m "futures-core",[m
[32m+[m[32m "futures-util",[m
[32m+[m[32m "mio",[m
[32m+[m[32m "num_cpus",[m
[32m+[m[32m "socket2",[m
[32m+[m[32m "tokio",[m
[32m+[m[32m "tracing",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "actix-service"[m
[32m+[m[32mversion = "2.0.2"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "3b894941f818cfdc7ccc4b9e60fa7e53b5042a2e8567270f9147d5591893373a"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "futures-core",[m
[32m+[m[32m "paste",[m
[32m+[m[32m "pin-project-lite",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "actix-utils"[m
[32m+[m[32mversion = "3.0.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "88a1dcdff1466e3c2488e1cb5c36a71822750ad43839937f85d2f4d9f8b705d8"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "local-waker",[m
[32m+[m[32m "pin-project-lite",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "actix-web"[m
[32m+[m[32mversion = "4.3.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "cd3cb42f9566ab176e1ef0b8b3a896529062b4efc6be0123046095914c4c1c96"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "actix-codec",[m
[32m+[m[32m "actix-http",[m
[32m+[m[32m "actix-macros",[m
[32m+[m[32m "actix-router",[m
[32m+[m[32m "actix-rt",[m
[32m+[m[32m "actix-server",[m
[32m+[m[32m "actix-service",[m
[32m+[m[32m "actix-utils",[m
[32m+[m[32m "actix-web-codegen",[m
[32m+[m[32m "ahash 0.7.6",[m
[32m+[m[32m "bytes",[m
[32m+[m[32m "bytestring",[m
[32m+[m[32m "cfg-if",[m
[32m+[m[32m "cookie",[m
[32m+[m[32m "derive_more",[m
[32m+[m[32m "encoding_rs",[m
[32m+[m[32m "futures-core",[m
[32m+[m[32m "futures-util",[m
[32m+[m[32m "http",[m
[32m+[m[32m "itoa",[m
[32m+[m[32m "language-tags",[m
[32m+[m[32m "log",[m
[32m+[m[32m "mime",[m
[32m+[m[32m "once_cell",[m
[32m+[m[32m "pin-project-lite",[m
[32m+[m[32m "regex",[m
[32m+[m[32m "serde",[m
[32m+[m[32m "serde_json",[m
[32m+[m[32m "serde_urlencoded",[m
[32m+[m[32m "smallvec",[m
[32m+[m[32m "socket2",[m
[32m+[m[32m "time",[m
[32m+[m[32m "url",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "actix-web-codegen"[m
[32m+[m[32mversion = "4.2.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "2262160a7ae29e3415554a3f1fc04c764b1540c116aa524683208078b7a75bc9"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "actix-router",[m
[32m+[m[32m "proc-macro2",[m
[32m+[m[32m "quote",[m
[32m+[m[32m "syn 1.0.109",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "addr2line"[m
[32m+[m[32mversion = "0.20.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "f4fa78e18c64fce05e902adecd7a5eed15a5e0a3439f7b0e169f0252214865e3"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "gimli",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "adler"[m
[32m+[m[32mversion = "1.0.2"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "f26201604c87b1e01bd3d98f8d5d9a8fcbb815e8cedb41ffccbeb4bf593a35fe"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "ahash"[m
[32m+[m[32mversion = "0.7.6"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "fcb51a0695d8f838b1ee009b3fbf66bda078cd64590202a864a8f3e8c4315c47"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "getrandom",[m
[32m+[m[32m "once_cell",[m
[32m+[m[32m "version_check",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "ahash"[m
[32m+[m[32mversion = "0.8.3"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "2c99f64d1e06488f620f932677e24bc6e2897582980441ae90a671415bd7ec2f"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "cfg-if",[m
[32m+[m[32m "getrandom",[m
[32m+[m[32m "once_cell",[m
[32m+[m[32m "version_check",[m
[32m+[m[32m][m
[32m+[m
 [[package]][m
 name = "aho-corasick"[m
 version = "1.0.2"[m
[36m@@ -11,6 +253,21 @@[m [mdependencies = [[m
  "memchr",[m
 ][m
 [m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "alloc-no-stdlib"[m
[32m+[m[32mversion = "2.0.4"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "cc7bb162ec39d46ab1ca8c77bf72e890535becd1751bb45f64c597edb4c8c6b3"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "alloc-stdlib"[m
[32m+[m[32mversion = "0.2.2"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "94fb8275041c72129eb51b7d0322c29b8387a0386127718b096429201a5d6ece"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "alloc-no-stdlib",[m
[32m+[m[32m][m
[32m+[m
 [[package]][m
 name = "anstream"[m
 version = "0.3.2"[m
[36m@@ -66,6 +323,29 @@[m [mversion = "1.0.71"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
 checksum = "9c7d0618f0e0b7e8ff11427422b64564d5fb0be1940354bfe2e0529b18a9d9b8"[m
 [m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "ascii"[m
[32m+[m[32mversion = "1.1.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "d92bec98840b8f03a5ff5413de5293bfcd8bf96467cf5452609f939ec6f5de16"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "askama_escape"[m
[32m+[m[32mversion = "0.10.3"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "619743e34b5ba4e9703bba34deac3427c72507c7159f5fd030aea8cac0cfe341"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "async-trait"[m
[32m+[m[32mversion = "0.1.71"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "a564d521dd56509c4c47480d00b80ee55f7e385ae48db5744c67ad50c92d2ebf"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "proc-macro2",[m
[32m+[m[32m "quote",[m
[32m+[m[32m "syn 2.0.26",[m
[32m+[m[32m][m
[32m+[m
 [[package]][m
 name = "atty"[m
 version = "0.2.14"[m
[36m@@ -83,17 +363,145 @@[m [mversion = "1.1.0"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
 checksum = "d468802bab17cbc0cc575e9b053f41e72aa36bfa6b7f55e3529ffa43161b97fa"[m
 [m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "axum"[m
[32m+[m[32mversion = "0.6.19"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "a6a1de45611fdb535bfde7b7de4fd54f4fd2b17b1737c0a59b69bf9b92074b8c"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "async-trait",[m
[32m+[m[32m "axum-core",[m
[32m+[m[32m "bitflags",[m
[32m+[m[32m "bytes",[m
[32m+[m[32m "futures-util",[m
[32m+[m[32m "http",[m
[32m+[m[32m "http-body",[m
[32m+[m[32m "hyper",[m
[32m+[m[32m "itoa",[m
[32m+[m[32m "matchit",[m
[32m+[m[32m "memchr",[m
[32m+[m[32m "mime",[m
[32m+[m[32m "percent-encoding",[m
[32m+[m[32m "pin-project-lite",[m
[32m+[m[32m "rustversion",[m
[32m+[m[32m "serde",[m
[32m+[m[32m "serde_json",[m
[32m+[m[32m "serde_path_to_error",[m
[32m+[m[32m "serde_urlencoded",[m
[32m+[m[32m "sync_wrapper",[m
[32m+[m[32m "tokio",[m
[32m+[m[32m "tower",[m
[32m+[m[32m "tower-layer",[m
[32m+[m[32m "tower-service",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "axum-core"[m
[32m+[m[32mversion = "0.3.4"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "759fa577a247914fd3f7f76d62972792636412fbfd634cd452f6a385a74d2d2c"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "async-trait",[m
[32m+[m[32m "bytes",[m
[32m+[m[32m "futures-util",[m
[32m+[m[32m "http",[m
[32m+[m[32m "http-body",[m
[32m+[m[32m "mime",[m
[32m+[m[32m "rustversion",[m
[32m+[m[32m "tower-layer",[m
[32m+[m[32m "tower-service",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "axum_static"[m
[32m+[m[32mversion = "1.2.2"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "85d257eb7e379c33bba9fc4d7871d517f33de266838ca15d467d6b040886c2e5"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "axum",[m
[32m+[m[32m "tower-http",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "backtrace"[m
[32m+[m[32mversion = "0.3.68"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "4319208da049c43661739c5fade2ba182f09d1dc2299b32298d3a31692b17e12"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "addr2line",[m
[32m+[m[32m "cc",[m
[32m+[m[32m "cfg-if",[m
[32m+[m[32m "libc",[m
[32m+[m[32m "miniz_oxide",[m
[32m+[m[32m "object",[m
[32m+[m[32m "rustc-demangle",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "base64"[m
[32m+[m[32mversion = "0.21.2"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "604178f6c5c21f02dc555784810edfb88d34ac2c73b2eae109655649ee73ce3d"[m
[32m+[m
 [[package]][m
 name = "bitflags"[m
 version = "1.3.2"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
 checksum = "bef38d45163c2f1dde094a7dfd33ccf595c92905c8f8f4fdc18d06fb1037718a"[m
 [m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "block-buffer"[m
[32m+[m[32mversion = "0.10.4"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "3078c7629b62d3f0439517fa394996acacc5cbc91c5a20d8c658e77abd503a71"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "generic-array",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "brotli"[m
[32m+[m[32mversion = "3.3.4"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "a1a0b1dbcc8ae29329621f8d4f0d835787c1c38bb1401979b49d13b0b305ff68"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "alloc-no-stdlib",[m
[32m+[m[32m "alloc-stdlib",[m
[32m+[m[32m "brotli-decompressor",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "brotli-decompressor"[m
[32m+[m[32mversion = "2.3.4"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "4b6561fd3f895a11e8f72af2cb7d22e08366bebc2b6b57f7744c4bda27034744"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "alloc-no-stdlib",[m
[32m+[m[32m "alloc-stdlib",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "bytes"[m
[32m+[m[32mversion = "1.4.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "89b2fd2a0dcf38d7971e2194b6b6eebab45ae01067456a7fd93d5547a61b70be"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "bytestring"[m
[32m+[m[32mversion = "1.3.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "238e4886760d98c4f899360c834fa93e62cf7f721ac3c2da375cbdf4b8679aae"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "bytes",[m
[32m+[m[32m][m
[32m+[m
 [[package]][m
 name = "cc"[m
 version = "1.0.79"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
 checksum = "50d30906286121d95be3d479533b458f87493b30a4b5f79a607db8f5d11aa91f"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "jobserver",[m
[32m+[m[32m][m
 [m
 [[package]][m
 name = "cfg-if"[m
[36m@@ -101,6 +509,12 @@[m [mversion = "1.0.0"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
 checksum = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd"[m
 [m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "chunked_transfer"[m
[32m+[m[32mversion = "1.4.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "cca491388666e04d7248af3f60f0c40cfb0991c72205595d7c396e3510207d1a"[m
[32m+[m
 [[package]][m
 name = "clap"[m
 version = "4.3.3"[m
[36m@@ -135,7 +549,7 @@[m [mdependencies = [[m
  "heck",[m
  "proc-macro2",[m
  "quote",[m
[31m- "syn",[m
[32m+[m[32m "syn 2.0.26",[m
 ][m
 [m
 [[package]][m
[36m@@ -174,6 +588,70 @@[m [mdependencies = [[m
  "windows-sys 0.45.0",[m
 ][m
 [m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "convert_case"[m
[32m+[m[32mversion = "0.4.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "6245d59a3e82a7fc217c5828a6692dbc6dfb63a0c8c90495621f7b9d79704a0e"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "cookie"[m
[32m+[m[32mversion = "0.16.2"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "e859cd57d0710d9e06c381b550c06e76992472a8c6d527aecd2fc673dcc231fb"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "percent-encoding",[m
[32m+[m[32m "time",[m
[32m+[m[32m "version_check",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "cpufeatures"[m
[32m+[m[32mversion = "0.2.9"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "a17b76ff3a4162b0b27f354a0c87015ddad39d35f9c0c36607a3bdd175dde1f1"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "libc",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "crc32fast"[m
[32m+[m[32mversion = "1.3.2"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "b540bd8bc810d3885c6ea91e2018302f68baba2129ab3e88f32389ee9370880d"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "cfg-if",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "crypto-common"[m
[32m+[m[32mversion = "0.1.6"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "1bfb12502f3fc46cca1bb51ac28df9d618d813cdc3d2f25b9fe775a34af26bb3"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "generic-array",[m
[32m+[m[32m "typenum",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "deranged"[m
[32m+[m[32mversion = "0.3.7"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "7684a49fb1af197853ef7b2ee694bc1f5b4179556f1e5710e1760c5db6f5e929"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "derive_more"[m
[32m+[m[32mversion = "0.99.17"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "4fb810d30a7c1953f91334de7244731fc3f3c10d7fe163338a35b9f640960321"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "convert_case",[m
[32m+[m[32m "proc-macro2",[m
[32m+[m[32m "quote",[m
[32m+[m[32m "rustc_version",[m
[32m+[m[32m "syn 1.0.109",[m
[32m+[m[32m][m
[32m+[m
 [[package]][m
 name = "dialoguer"[m
 version = "0.10.4"[m
[36m@@ -186,6 +664,16 @@[m [mdependencies = [[m
  "zeroize",[m
 ][m
 [m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "digest"[m
[32m+[m[32mversion = "0.10.7"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "9ed9a281f7bc9b7576e61468ba615a66a5c8cfdff42420a70aa82701a3b1e292"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "block-buffer",[m
[32m+[m[32m "crypto-common",[m
[32m+[m[32m][m
[32m+[m
 [[package]][m
 name = "encode_unicode"[m
 version = "0.3.6"[m
[36m@@ -193,273 +681,1171 @@[m [msource = "registry+https://github.com/rust-lang/crates.io-index"[m
 checksum = "a357d28ed41a50f9c765dbfe56cbc04a64e53e5fc58ba79fbc34c10ef3df831f"[m
 [m
 [[package]][m
[31m-name = "errno"[m
[31m-version = "0.3.1"[m
[32m+[m[32mname = "encoding_rs"[m
[32m+[m[32mversion = "0.8.32"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "071a31f4ee85403370b58aca746f01041ede6f0da2730960ad001edc2b71b394"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "cfg-if",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "equivalent"[m
[32m+[m[32mversion = "1.0.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "5443807d6dff69373d433ab9ef5378ad8df50ca6298caf15de6e52e24aaf54d5"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "errno"[m
[32m+[m[32mversion = "0.3.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "4bcfec3a70f97c962c307b2d2c56e358cf1d00b558d74262b5f929ee8cc7e73a"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "errno-dragonfly",[m
[32m+[m[32m "libc",[m
[32m+[m[32m "windows-sys 0.48.0",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "errno-dragonfly"[m
[32m+[m[32mversion = "0.1.2"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "aa68f1b12764fab894d2755d2518754e71b4fd80ecfb822714a1206c2aab39bf"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "cc",[m
[32m+[m[32m "libc",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "fastrand"[m
[32m+[m[32mversion = "1.9.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "e51093e27b0797c359783294ca4f0a911c270184cb10f85783b118614a1501be"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "instant",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "file-serve"[m
[32m+[m[32mversion = "0.2.4"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "a7d3d0bd7a7c70bb7544df361198348a45d9d3f50cf03ccd2745846557061a2f"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "log",[m
[32m+[m[32m "mime_guess",[m
[32m+[m[32m "tiny_http",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "flate2"[m
[32m+[m[32mversion = "1.0.26"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "3b9429470923de8e8cbd4d2dc513535400b4b3fef0319fb5c4e1f520a7bef743"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "crc32fast",[m
[32m+[m[32m "miniz_oxide",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "fnv"[m
[32m+[m[32mversion = "1.0.7"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "3f9eec918d3f24069decb9af1554cad7c880e2da24a9afd88aca000531ab82c1"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "form_urlencoded"[m
[32m+[m[32mversion = "1.2.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "a62bc1cf6f830c2ec14a513a9fb124d0a213a629668a4186f329db21fe045652"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "percent-encoding",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "futures-channel"[m
[32m+[m[32mversion = "0.3.28"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "955518d47e09b25bbebc7a18df10b81f0c766eaf4c4f1cccef2fca5f2a4fb5f2"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "futures-core",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "futures-core"[m
[32m+[m[32mversion = "0.3.28"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "4bca583b7e26f571124fe5b7561d49cb2868d79116cfa0eefce955557c6fee8c"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "futures-sink"[m
[32m+[m[32mversion = "0.3.28"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "f43be4fe21a13b9781a69afa4985b0f6ee0e1afab2c6f454a8cf30e2b2237b6e"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "futures-task"[m
[32m+[m[32mversion = "0.3.28"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "76d3d132be6c0e6aa1534069c705a74a5997a356c0dc2f86a47765e5617c5b65"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "futures-util"[m
[32m+[m[32mversion = "0.3.28"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "26b01e40b772d54cf6c6d721c1d1abd0647a0106a12ecaa1c186273392a69533"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "futures-core",[m
[32m+[m[32m "futures-task",[m
[32m+[m[32m "pin-project-lite",[m
[32m+[m[32m "pin-utils",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "generic-array"[m
[32m+[m[32mversion = "0.14.7"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "85649ca51fd72272d7821adaf274ad91c288277713d9c18820d8499a7ff69e9a"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "typenum",[m
[32m+[m[32m "version_check",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "getrandom"[m
[32m+[m[32mversion = "0.2.10"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "be4136b2a15dd319360be1c07d9933517ccf0be8f16bf62a3bee4f0d618df427"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "cfg-if",[m
[32m+[m[32m "libc",[m
[32m+[m[32m "wasi",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "gimli"[m
[32m+[m[32mversion = "0.27.3"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "b6c80984affa11d98d1b88b66ac8853f143217b399d3c74116778ff8fdb4ed2e"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "golb"[m
[32m+[m[32mversion = "0.1.0"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "actix-files",[m
[32m+[m[32m "actix-web",[m
[32m+[m[32m "anyhow",[m
[32m+[m[32m "axum",[m
[32m+[m[32m "axum_static",[m
[32m+[m[32m "clap",[m
[32m+[m[32m "colored",[m
[32m+[m[32m "dialoguer",[m
[32m+[m[32m "file-serve",[m
[32m+[m[32m "human-panic",[m
[32m+[m[32m "indicatif",[m
[32m+[m[32m "markdown",[m
[32m+[m[32m "tokio",[m
[32m+[m[32m "upon",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "h2"[m
[32m+[m[32mversion = "0.3.20"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "97ec8491ebaf99c8eaa73058b045fe58073cd6be7f596ac993ced0b0a0c01049"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "bytes",[m
[32m+[m[32m "fnv",[m
[32m+[m[32m "futures-core",[m
[32m+[m[32m "futures-sink",[m
[32m+[m[32m "futures-util",[m
[32m+[m[32m "http",[m
[32m+[m[32m "indexmap 1.9.3",[m
[32m+[m[32m "slab",[m
[32m+[m[32m "tokio",[m
[32m+[m[32m "tokio-util",[m
[32m+[m[32m "tracing",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "hashbrown"[m
[32m+[m[32mversion = "0.12.3"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "8a9ee70c43aaf417c914396645a0fa852624801b24ebb7ae78fe8272889ac888"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "hashbrown"[m
[32m+[m[32mversion = "0.14.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "2c6201b9ff9fd90a5a3bac2e56a830d0caa509576f0e503818ee82c181b3437a"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "heck"[m
[32m+[m[32mversion = "0.4.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "95505c38b4572b2d910cecb0281560f54b440a19336cbbcb27bf6ce6adc6f5a8"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "hermit-abi"[m
[32m+[m[32mversion = "0.1.19"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "62b467343b94ba476dcb2500d242dadbb39557df889310ac77c5d99100aaac33"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "libc",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "hermit-abi"[m
[32m+[m[32mversion = "0.3.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "fed44880c466736ef9a5c5b5facefb5ed0785676d0c02d612db14e54f0d84286"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "http"[m
[32m+[m[32mversion = "0.2.9"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "bd6effc99afb63425aff9b05836f029929e345a6148a14b7ecd5ab67af944482"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "bytes",[m
[32m+[m[32m "fnv",[m
[32m+[m[32m "itoa",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "http-body"[m
[32m+[m[32mversion = "0.4.5"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "d5f38f16d184e36f2408a55281cd658ecbd3ca05cce6d6510a176eca393e26d1"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "bytes",[m
[32m+[m[32m "http",[m
[32m+[m[32m "pin-project-lite",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "http-range"[m
[32m+[m[32mversion = "0.1.5"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "21dec9db110f5f872ed9699c3ecf50cf16f423502706ba5c72462e28d3157573"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "http-range-header"[m
[32m+[m[32mversion = "0.3.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "0bfe8eed0a9285ef776bb792479ea3834e8b94e13d615c2f66d03dd50a435a29"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "httparse"[m
[32m+[m[32mversion = "1.8.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "d897f394bad6a705d5f4104762e116a75639e470d80901eed05a860a95cb1904"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "httpdate"[m
[32m+[m[32mversion = "1.0.2"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "c4a1e36c821dbe04574f602848a19f742f4fb3c98d40449f11bcad18d6b17421"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "human-panic"[m
[32m+[m[32mversion = "1.1.5"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "38a841f87949b0dd751864e769a870be79dc34abcee1cf31d737a61d498b22b6"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "anstream",[m
[32m+[m[32m "anstyle",[m
[32m+[m[32m "backtrace",[m
[32m+[m[32m "os_info",[m
[32m+[m[32m "serde",[m
[32m+[m[32m "serde_derive",[m
[32m+[m[32m "toml",[m
[32m+[m[32m "uuid",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "hyper"[m
[32m+[m[32mversion = "0.14.27"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "ffb1cfd654a8219eaef89881fdb3bb3b1cdc5fa75ded05d6933b2b382e395468"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "bytes",[m
[32m+[m[32m "futures-channel",[m
[32m+[m[32m "futures-core",[m
[32m+[m[32m "futures-util",[m
[32m+[m[32m "http",[m
[32m+[m[32m "http-body",[m
[32m+[m[32m "httparse",[m
[32m+[m[32m "httpdate",[m
[32m+[m[32m "itoa",[m
[32m+[m[32m "pin-project-lite",[m
[32m+[m[32m "socket2",[m
[32m+[m[32m "tokio",[m
[32m+[m[32m "tower-service",[m
[32m+[m[32m "tracing",[m
[32m+[m[32m "want",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "idna"[m
[32m+[m[32mversion = "0.4.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "7d20d6b07bfbc108882d88ed8e37d39636dcc260e15e30c45e6ba089610b917c"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "unicode-bidi",[m
[32m+[m[32m "unicode-normalization",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "indexmap"[m
[32m+[m[32mversion = "1.9.3"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "bd070e393353796e801d209ad339e89596eb4c8d430d18ede6a1cced8fafbd99"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "autocfg",[m
[32m+[m[32m "hashbrown 0.12.3",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "indexmap"[m
[32m+[m[32mversion = "2.0.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "d5477fe2230a79769d8dc68e0eabf5437907c0457a5614a9e8dddb67f65eb65d"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "equivalent",[m
[32m+[m[32m "hashbrown 0.14.0",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "indicatif"[m
[32m+[m[32mversion = "0.17.5"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "8ff8cc23a7393a397ed1d7f56e6365cba772aba9f9912ab968b03043c395d057"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "console",[m
[32m+[m[32m "instant",[m
[32m+[m[32m "number_prefix",[m
[32m+[m[32m "portable-atomic",[m
[32m+[m[32m "unicode-width",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "instant"[m
[32m+[m[32mversion = "0.1.12"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "7a5bbe824c507c5da5956355e86a746d82e0e1464f65d862cc5e71da70e94b2c"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "cfg-if",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "io-lifetimes"[m
[32m+[m[32mversion = "1.0.11"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "eae7b9aee968036d54dce06cebaefd919e4472e753296daccd6d344e3e2df0c2"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "hermit-abi 0.3.1",[m
[32m+[m[32m "libc",[m
[32m+[m[32m "windows-sys 0.48.0",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "is-terminal"[m
[32m+[m[32mversion = "0.4.7"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "adcf93614601c8129ddf72e2d5633df827ba6551541c6d8c59520a371475be1f"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "hermit-abi 0.3.1",[m
[32m+[m[32m "io-lifetimes",[m
[32m+[m[32m "rustix",[m
[32m+[m[32m "windows-sys 0.48.0",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "itoa"[m
[32m+[m[32mversion = "1.0.9"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "af150ab688ff2122fcef229be89cb50dd66af9e01a4ff320cc137eecc9bacc38"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "jobserver"[m
[32m+[m[32mversion = "0.1.26"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "936cfd212a0155903bcbc060e316fb6cc7cbf2e1907329391ebadc1fe0ce77c2"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "libc",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "language-tags"[m
[32m+[m[32mversion = "0.3.2"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "d4345964bb142484797b161f473a503a434de77149dd8c7427788c6e13379388"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "lazy_static"[m
[32m+[m[32mversion = "1.4.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "libc"[m
[32m+[m[32mversion = "0.2.146"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "f92be4933c13fd498862a9e02a3055f8a8d9c039ce33db97306fd5a6caa7f29b"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "linux-raw-sys"[m
[32m+[m[32mversion = "0.3.8"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "ef53942eb7bf7ff43a617b3e2c1c4a5ecf5944a7c1bc12d7ee39bbb15e5c1519"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "local-channel"[m
[32m+[m[32mversion = "0.1.3"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "7f303ec0e94c6c54447f84f3b0ef7af769858a9c4ef56ef2a986d3dcd4c3fc9c"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "futures-core",[m
[32m+[m[32m "futures-sink",[m
[32m+[m[32m "futures-util",[m
[32m+[m[32m "local-waker",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "local-waker"[m
[32m+[m[32mversion = "0.1.3"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "e34f76eb3611940e0e7d53a9aaa4e6a3151f69541a282fd0dad5571420c53ff1"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "lock_api"[m
[32m+[m[32mversion = "0.4.10"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "c1cc9717a20b1bb222f333e6a92fd32f7d8a18ddc5a3191a11af45dcbf4dcd16"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "autocfg",[m
[32m+[m[32m "scopeguard",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "log"[m
[32m+[m[32mversion = "0.4.19"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "b06a4cde4c0f271a446782e3eff8de789548ce57dbc8eca9292c27f4a42004b4"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "markdown"[m
[32m+[m[32mversion = "0.3.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "ef3aab6a1d529b112695f72beec5ee80e729cb45af58663ec902c8fac764ecdd"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "lazy_static",[m
[32m+[m[32m "pipeline",[m
[32m+[m[32m "regex",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "matchit"[m
[32m+[m[32mversion = "0.7.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "b87248edafb776e59e6ee64a79086f65890d3510f2c656c000bf2a7e8a0aea40"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "memchr"[m
[32m+[m[32mversion = "2.5.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "2dffe52ecf27772e601905b7522cb4ef790d2cc203488bbd0e2fe85fcb74566d"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "mime"[m
[32m+[m[32mversion = "0.3.17"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "6877bb514081ee2a7ff5ef9de3281f14a4dd4bceac4c09388074a6b5df8a139a"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "mime_guess"[m
[32m+[m[32mversion = "2.0.4"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "4192263c238a5f0d0c6bfd21f336a313a4ce1c450542449ca191bb657b4642ef"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "mime",[m
[32m+[m[32m "unicase",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "miniz_oxide"[m
[32m+[m[32mversion = "0.7.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "e7810e0be55b428ada41041c41f32c9f1a42817901b4ccf45fa3d4b6561e74c7"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "adler",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "mio"[m
[32m+[m[32mversion = "0.8.8"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "927a765cd3fc26206e66b296465fa9d3e5ab003e651c1b3c060e7956d96b19d2"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "libc",[m
[32m+[m[32m "log",[m
[32m+[m[32m "wasi",[m
[32m+[m[32m "windows-sys 0.48.0",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "num_cpus"[m
[32m+[m[32mversion = "1.16.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "4161fcb6d602d4d2081af7c3a45852d875a03dd337a6bfdd6e06407b61342a43"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "hermit-abi 0.3.1",[m
[32m+[m[32m "libc",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "number_prefix"[m
[32m+[m[32mversion = "0.4.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "830b246a0e5f20af87141b25c173cd1b609bd7779a4617d6ec582abaf90870f3"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "object"[m
[32m+[m[32mversion = "0.31.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "8bda667d9f2b5051b8833f59f3bf748b28ef54f850f4fcb389a252aa383866d1"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "memchr",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "once_cell"[m
[32m+[m[32mversion = "1.18.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "dd8b5dd2ae5ed71462c540258bedcb51965123ad7e7ccf4b9a8cafaa4a63576d"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "os_info"[m
[32m+[m[32mversion = "3.7.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "006e42d5b888366f1880eda20371fedde764ed2213dc8496f49622fa0c99cd5e"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "log",[m
[32m+[m[32m "serde",[m
[32m+[m[32m "winapi",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "parking_lot"[m
[32m+[m[32mversion = "0.12.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "3742b2c103b9f06bc9fff0a37ff4912935851bee6d36f3c02bcc755bcfec228f"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "lock_api",[m
[32m+[m[32m "parking_lot_core",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "parking_lot_core"[m
[32m+[m[32mversion = "0.9.8"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "93f00c865fe7cabf650081affecd3871070f26767e7b2070a3ffae14c654b447"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "cfg-if",[m
[32m+[m[32m "libc",[m
[32m+[m[32m "redox_syscall",[m
[32m+[m[32m "smallvec",[m
[32m+[m[32m "windows-targets 0.48.0",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "paste"[m
[32m+[m[32mversion = "1.0.14"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "de3145af08024dea9fa9914f381a17b8fc6034dfb00f3a84013f7ff43f29ed4c"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "percent-encoding"[m
[32m+[m[32mversion = "2.3.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "9b2a4787296e9989611394c33f193f676704af1686e70b8f8033ab5ba9a35a94"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "pin-project"[m
[32m+[m[32mversion = "1.1.2"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "030ad2bc4db10a8944cb0d837f158bdfec4d4a4873ab701a95046770d11f8842"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "pin-project-internal",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "pin-project-internal"[m
[32m+[m[32mversion = "1.1.2"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "ec2e072ecce94ec471b13398d5402c188e76ac03cf74dd1a975161b23a3f6d9c"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "proc-macro2",[m
[32m+[m[32m "quote",[m
[32m+[m[32m "syn 2.0.26",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "pin-project-lite"[m
[32m+[m[32mversion = "0.2.10"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "4c40d25201921e5ff0c862a505c6557ea88568a4e3ace775ab55e93f2f4f9d57"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "pin-utils"[m
[32m+[m[32mversion = "0.1.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "8b870d8c151b6f2fb93e84a13146138f05d02ed11c7e7c54f8826aaaf7c9f184"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "pipeline"[m
[32m+[m[32mversion = "0.5.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "d15b6607fa632996eb8a17c9041cb6071cb75ac057abd45dece578723ea8c7c0"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "pkg-config"[m
[32m+[m[32mversion = "0.3.27"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "26072860ba924cbfa98ea39c8c19b4dd6a4a25423dbdf219c1eca91aa0cf6964"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "portable-atomic"[m
[32m+[m[32mversion = "1.3.3"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "767eb9f07d4a5ebcb39bbf2d452058a93c011373abf6832e24194a1c3f004794"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "ppv-lite86"[m
[32m+[m[32mversion = "0.2.17"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "5b40af805b3121feab8a3c29f04d8ad262fa8e0561883e7653e024ae4479e6de"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "proc-macro2"[m
[32m+[m[32mversion = "1.0.66"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "18fb31db3f9bddb2ea821cde30a9f70117e3f119938b5ee630b7403aa6e2ead9"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "unicode-ident",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "quote"[m
[32m+[m[32mversion = "1.0.31"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "5fe8a65d69dd0808184ebb5f836ab526bb259db23c657efa38711b1072ee47f0"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "proc-macro2",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "rand"[m
[32m+[m[32mversion = "0.8.5"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "34af8d1a0e25924bc5b7c43c079c942339d8f0a8b57c39049bef581b46327404"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "libc",[m
[32m+[m[32m "rand_chacha",[m
[32m+[m[32m "rand_core",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "rand_chacha"[m
[32m+[m[32mversion = "0.3.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "e6c10a63a0fa32252be49d21e7709d4d4baf8d231c2dbce1eaa8141b9b127d88"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "ppv-lite86",[m
[32m+[m[32m "rand_core",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "rand_core"[m
[32m+[m[32mversion = "0.6.4"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "ec0be4795e2f6a28069bec0b5ff3e2ac9bafc99e6a9a7dc3547996c5c816922c"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "getrandom",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "redox_syscall"[m
[32m+[m[32mversion = "0.3.5"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "567664f262709473930a4bf9e51bf2ebf3348f2e748ccc50dea20646858f8f29"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "bitflags",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "regex"[m
[32m+[m[32mversion = "1.8.4"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "d0ab3ca65655bb1e41f2a8c8cd662eb4fb035e67c3f78da1d61dffe89d07300f"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "aho-corasick",[m
[32m+[m[32m "memchr",[m
[32m+[m[32m "regex-syntax",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "regex-syntax"[m
[32m+[m[32mversion = "0.7.2"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "436b050e76ed2903236f032a59761c1eb99e1b0aead2c257922771dab1fc8c78"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "rustc-demangle"[m
[32m+[m[32mversion = "0.1.23"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "d626bb9dae77e28219937af045c257c28bfd3f69333c512553507f5f9798cb76"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "rustc_version"[m
[32m+[m[32mversion = "0.4.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "bfa0f585226d2e68097d4f95d113b15b83a82e819ab25717ec0590d9584ef366"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "semver",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "rustix"[m
[32m+[m[32mversion = "0.37.19"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "acf8729d8542766f1b2cf77eb034d52f40d375bb8b615d0b147089946e16613d"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "bitflags",[m
[32m+[m[32m "errno",[m
[32m+[m[32m "io-lifetimes",[m
[32m+[m[32m "libc",[m
[32m+[m[32m "linux-raw-sys",[m
[32m+[m[32m "windows-sys 0.48.0",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "rustversion"[m
[32m+[m[32mversion = "1.0.14"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "7ffc183a10b4478d04cbbbfc96d0873219d962dd5accaff2ffbd4ceb7df837f4"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "ryu"[m
[32m+[m[32mversion = "1.0.15"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "1ad4cc8da4ef723ed60bced201181d83791ad433213d8c24efffda1eec85d741"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "scopeguard"[m
[32m+[m[32mversion = "1.1.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "d29ab0c6d3fc0ee92fe66e2d99f700eab17a8d57d1c1d3b748380fb20baa78cd"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "semver"[m
[32m+[m[32mversion = "1.0.18"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "b0293b4b29daaf487284529cc2f5675b8e57c61f70167ba415a463651fd6a918"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "serde"[m
[32m+[m[32mversion = "1.0.173"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "e91f70896d6720bc714a4a57d22fc91f1db634680e65c8efe13323f1fa38d53f"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "serde_derive",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "serde_derive"[m
[32m+[m[32mversion = "1.0.173"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "a6250dde8342e0232232be9ca3db7aa40aceb5a3e5dd9bddbc00d99a007cde49"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "proc-macro2",[m
[32m+[m[32m "quote",[m
[32m+[m[32m "syn 2.0.26",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "serde_json"[m
[32m+[m[32mversion = "1.0.103"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "d03b412469450d4404fe8499a268edd7f8b79fecb074b0d812ad64ca21f4031b"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "itoa",[m
[32m+[m[32m "ryu",[m
[32m+[m[32m "serde",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "serde_path_to_error"[m
[32m+[m[32mversion = "0.1.14"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "4beec8bce849d58d06238cb50db2e1c417cfeafa4c63f692b15c82b7c80f8335"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "itoa",[m
[32m+[m[32m "serde",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "serde_spanned"[m
[32m+[m[32mversion = "0.6.3"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "96426c9936fd7a0124915f9185ea1d20aa9445cc9821142f0a73bc9207a2e186"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "serde",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "serde_urlencoded"[m
[32m+[m[32mversion = "0.7.1"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "4bcfec3a70f97c962c307b2d2c56e358cf1d00b558d74262b5f929ee8cc7e73a"[m
[32m+[m[32mchecksum = "d3491c14715ca2294c4d6a88f15e84739788c1d030eed8c110436aafdaa2f3fd"[m
 dependencies = [[m
[31m- "errno-dragonfly",[m
[31m- "libc",[m
[31m- "windows-sys 0.48.0",[m
[32m+[m[32m "form_urlencoded",[m
[32m+[m[32m "itoa",[m
[32m+[m[32m "ryu",[m
[32m+[m[32m "serde",[m
 ][m
 [m
 [[package]][m
[31m-name = "errno-dragonfly"[m
[31m-version = "0.1.2"[m
[32m+[m[32mname = "sha1"[m
[32m+[m[32mversion = "0.10.5"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "aa68f1b12764fab894d2755d2518754e71b4fd80ecfb822714a1206c2aab39bf"[m
[32m+[m[32mchecksum = "f04293dc80c3993519f2d7f6f511707ee7094fe0c6d3406feb330cdb3540eba3"[m
 dependencies = [[m
[31m- "cc",[m
[31m- "libc",[m
[32m+[m[32m "cfg-if",[m
[32m+[m[32m "cpufeatures",[m
[32m+[m[32m "digest",[m
 ][m
 [m
 [[package]][m
[31m-name = "fastrand"[m
[31m-version = "1.9.0"[m
[32m+[m[32mname = "shell-words"[m
[32m+[m[32mversion = "1.1.0"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "e51093e27b0797c359783294ca4f0a911c270184cb10f85783b118614a1501be"[m
[32m+[m[32mchecksum = "24188a676b6ae68c3b2cb3a01be17fbf7240ce009799bb56d5b1409051e78fde"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "signal-hook-registry"[m
[32m+[m[32mversion = "1.4.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "d8229b473baa5980ac72ef434c4415e70c4b5e71b423043adb4ba059f89c99a1"[m
 dependencies = [[m
[31m- "instant",[m
[32m+[m[32m "libc",[m
 ][m
 [m
 [[package]][m
[31m-name = "golb"[m
[31m-version = "0.1.0"[m
[32m+[m[32mname = "slab"[m
[32m+[m[32mversion = "0.4.8"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "6528351c9bc8ab22353f9d776db39a20288e8d6c37ef8cfe3317cf875eecfc2d"[m
 dependencies = [[m
[31m- "anyhow",[m
[31m- "clap",[m
[31m- "colored",[m
[31m- "dialoguer",[m
[31m- "indicatif",[m
[31m- "markdown",[m
[31m- "upon",[m
[32m+[m[32m "autocfg",[m
 ][m
 [m
 [[package]][m
[31m-name = "heck"[m
[31m-version = "0.4.1"[m
[32m+[m[32mname = "smallvec"[m
[32m+[m[32mversion = "1.11.0"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "95505c38b4572b2d910cecb0281560f54b440a19336cbbcb27bf6ce6adc6f5a8"[m
[32m+[m[32mchecksum = "62bb4feee49fdd9f707ef802e22365a35de4b7b299de4763d44bfea899442ff9"[m
 [m
 [[package]][m
[31m-name = "hermit-abi"[m
[31m-version = "0.1.19"[m
[32m+[m[32mname = "socket2"[m
[32m+[m[32mversion = "0.4.9"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "62b467343b94ba476dcb2500d242dadbb39557df889310ac77c5d99100aaac33"[m
[32m+[m[32mchecksum = "64a4a911eed85daf18834cfaa86a79b7d266ff93ff5ba14005426219480ed662"[m
 dependencies = [[m
  "libc",[m
[32m+[m[32m "winapi",[m
 ][m
 [m
 [[package]][m
[31m-name = "hermit-abi"[m
[31m-version = "0.3.1"[m
[32m+[m[32mname = "strsim"[m
[32m+[m[32mversion = "0.10.0"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "fed44880c466736ef9a5c5b5facefb5ed0785676d0c02d612db14e54f0d84286"[m
[32m+[m[32mchecksum = "73473c0e59e6d5812c5dfe2a064a6444949f089e20eec9a2e5506596494e4623"[m
 [m
 [[package]][m
[31m-name = "indicatif"[m
[31m-version = "0.17.5"[m
[32m+[m[32mname = "syn"[m
[32m+[m[32mversion = "1.0.109"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "8ff8cc23a7393a397ed1d7f56e6365cba772aba9f9912ab968b03043c395d057"[m
[32m+[m[32mchecksum = "72b64191b275b66ffe2469e8af2c1cfe3bafa67b529ead792a6d0160888b4237"[m
 dependencies = [[m
[31m- "console",[m
[31m- "instant",[m
[31m- "number_prefix",[m
[31m- "portable-atomic",[m
[31m- "unicode-width",[m
[32m+[m[32m "proc-macro2",[m
[32m+[m[32m "quote",[m
[32m+[m[32m "unicode-ident",[m
 ][m
 [m
 [[package]][m
[31m-name = "instant"[m
[31m-version = "0.1.12"[m
[32m+[m[32mname = "syn"[m
[32m+[m[32mversion = "2.0.26"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "7a5bbe824c507c5da5956355e86a746d82e0e1464f65d862cc5e71da70e94b2c"[m
[32m+[m[32mchecksum = "45c3457aacde3c65315de5031ec191ce46604304d2446e803d71ade03308d970"[m
 dependencies = [[m
[31m- "cfg-if",[m
[32m+[m[32m "proc-macro2",[m
[32m+[m[32m "quote",[m
[32m+[m[32m "unicode-ident",[m
 ][m
 [m
 [[package]][m
[31m-name = "io-lifetimes"[m
[31m-version = "1.0.11"[m
[32m+[m[32mname = "sync_wrapper"[m
[32m+[m[32mversion = "0.1.2"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "eae7b9aee968036d54dce06cebaefd919e4472e753296daccd6d344e3e2df0c2"[m
[32m+[m[32mchecksum = "2047c6ded9c721764247e62cd3b03c09ffc529b2ba5b10ec482ae507a4a70160"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "tempfile"[m
[32m+[m[32mversion = "3.6.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "31c0432476357e58790aaa47a8efb0c5138f137343f3b5f23bd36a27e3b0a6d6"[m
 dependencies = [[m
[31m- "hermit-abi 0.3.1",[m
[31m- "libc",[m
[32m+[m[32m "autocfg",[m
[32m+[m[32m "cfg-if",[m
[32m+[m[32m "fastrand",[m
[32m+[m[32m "redox_syscall",[m
[32m+[m[32m "rustix",[m
  "windows-sys 0.48.0",[m
 ][m
 [m
 [[package]][m
[31m-name = "is-terminal"[m
[31m-version = "0.4.7"[m
[32m+[m[32mname = "time"[m
[32m+[m[32mversion = "0.3.25"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "adcf93614601c8129ddf72e2d5633df827ba6551541c6d8c59520a371475be1f"[m
[32m+[m[32mchecksum = "b0fdd63d58b18d663fbdf70e049f00a22c8e42be082203be7f26589213cd75ea"[m
 dependencies = [[m
[31m- "hermit-abi 0.3.1",[m
[31m- "io-lifetimes",[m
[31m- "rustix",[m
[31m- "windows-sys 0.48.0",[m
[32m+[m[32m "deranged",[m
[32m+[m[32m "itoa",[m
[32m+[m[32m "serde",[m
[32m+[m[32m "time-core",[m
[32m+[m[32m "time-macros",[m
 ][m
 [m
 [[package]][m
[31m-name = "lazy_static"[m
[31m-version = "1.4.0"[m
[32m+[m[32mname = "time-core"[m
[32m+[m[32mversion = "0.1.1"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646"[m
[32m+[m[32mchecksum = "7300fbefb4dadc1af235a9cef3737cea692a9d97e1b9cbcd4ebdae6f8868e6fb"[m
 [m
 [[package]][m
[31m-name = "libc"[m
[31m-version = "0.2.146"[m
[32m+[m[32mname = "time-macros"[m
[32m+[m[32mversion = "0.2.11"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "f92be4933c13fd498862a9e02a3055f8a8d9c039ce33db97306fd5a6caa7f29b"[m
[32m+[m[32mchecksum = "eb71511c991639bb078fd5bf97757e03914361c48100d52878b8e52b46fb92cd"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "time-core",[m
[32m+[m[32m][m
 [m
 [[package]][m
[31m-name = "linux-raw-sys"[m
[31m-version = "0.3.8"[m
[32m+[m[32mname = "tiny_http"[m
[32m+[m[32mversion = "0.12.0"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "ef53942eb7bf7ff43a617b3e2c1c4a5ecf5944a7c1bc12d7ee39bbb15e5c1519"[m
[32m+[m[32mchecksum = "389915df6413a2e74fb181895f933386023c71110878cd0825588928e64cdc82"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "ascii",[m
[32m+[m[32m "chunked_transfer",[m
[32m+[m[32m "httpdate",[m
[32m+[m[32m "log",[m
[32m+[m[32m][m
 [m
 [[package]][m
[31m-name = "markdown"[m
[31m-version = "0.3.0"[m
[32m+[m[32mname = "tinyvec"[m
[32m+[m[32mversion = "1.6.0"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "ef3aab6a1d529b112695f72beec5ee80e729cb45af58663ec902c8fac764ecdd"[m
[32m+[m[32mchecksum = "87cc5ceb3875bb20c2890005a4e226a4651264a5c75edb2421b52861a0a0cb50"[m
 dependencies = [[m
[31m- "lazy_static",[m
[31m- "pipeline",[m
[31m- "regex",[m
[32m+[m[32m "tinyvec_macros",[m
 ][m
 [m
 [[package]][m
[31m-name = "memchr"[m
[31m-version = "2.5.0"[m
[32m+[m[32mname = "tinyvec_macros"[m
[32m+[m[32mversion = "0.1.1"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "2dffe52ecf27772e601905b7522cb4ef790d2cc203488bbd0e2fe85fcb74566d"[m
[32m+[m[32mchecksum = "1f3ccbac311fea05f86f61904b462b55fb3df8837a366dfc601a0161d0532f20"[m
 [m
 [[package]][m
[31m-name = "number_prefix"[m
[31m-version = "0.4.0"[m
[32m+[m[32mname = "tokio"[m
[32m+[m[32mversion = "1.29.1"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "830b246a0e5f20af87141b25c173cd1b609bd7779a4617d6ec582abaf90870f3"[m
[32m+[m[32mchecksum = "532826ff75199d5833b9d2c5fe410f29235e25704ee5f0ef599fb51c21f4a4da"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "autocfg",[m
[32m+[m[32m "backtrace",[m
[32m+[m[32m "bytes",[m
[32m+[m[32m "libc",[m
[32m+[m[32m "mio",[m
[32m+[m[32m "num_cpus",[m
[32m+[m[32m "parking_lot",[m
[32m+[m[32m "pin-project-lite",[m
[32m+[m[32m "signal-hook-registry",[m
[32m+[m[32m "socket2",[m
[32m+[m[32m "tokio-macros",[m
[32m+[m[32m "windows-sys 0.48.0",[m
[32m+[m[32m][m
 [m
 [[package]][m
[31m-name = "once_cell"[m
[31m-version = "1.18.0"[m
[32m+[m[32mname = "tokio-macros"[m
[32m+[m[32mversion = "2.1.0"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "dd8b5dd2ae5ed71462c540258bedcb51965123ad7e7ccf4b9a8cafaa4a63576d"[m
[32m+[m[32mchecksum = "630bdcf245f78637c13ec01ffae6187cca34625e8c63150d424b59e55af2675e"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "proc-macro2",[m
[32m+[m[32m "quote",[m
[32m+[m[32m "syn 2.0.26",[m
[32m+[m[32m][m
 [m
 [[package]][m
[31m-name = "pipeline"[m
[31m-version = "0.5.0"[m
[32m+[m[32mname = "tokio-util"[m
[32m+[m[32mversion = "0.7.8"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "d15b6607fa632996eb8a17c9041cb6071cb75ac057abd45dece578723ea8c7c0"[m
[32m+[m[32mchecksum = "806fe8c2c87eccc8b3267cbae29ed3ab2d0bd37fca70ab622e46aaa9375ddb7d"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "bytes",[m
[32m+[m[32m "futures-core",[m
[32m+[m[32m "futures-sink",[m
[32m+[m[32m "pin-project-lite",[m
[32m+[m[32m "tokio",[m
[32m+[m[32m "tracing",[m
[32m+[m[32m][m
 [m
 [[package]][m
[31m-name = "portable-atomic"[m
[31m-version = "1.3.3"[m
[32m+[m[32mname = "toml"[m
[32m+[m[32mversion = "0.7.6"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "767eb9f07d4a5ebcb39bbf2d452058a93c011373abf6832e24194a1c3f004794"[m
[32m+[m[32mchecksum = "c17e963a819c331dcacd7ab957d80bc2b9a9c1e71c804826d2f283dd65306542"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "serde",[m
[32m+[m[32m "serde_spanned",[m
[32m+[m[32m "toml_datetime",[m
[32m+[m[32m "toml_edit",[m
[32m+[m[32m][m
 [m
 [[package]][m
[31m-name = "proc-macro2"[m
[31m-version = "1.0.60"[m
[32m+[m[32mname = "toml_datetime"[m
[32m+[m[32mversion = "0.6.3"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "dec2b086b7a862cf4de201096214fa870344cf922b2b30c167badb3af3195406"[m
[32m+[m[32mchecksum = "7cda73e2f1397b1262d6dfdcef8aafae14d1de7748d66822d3bfeeb6d03e5e4b"[m
 dependencies = [[m
[31m- "unicode-ident",[m
[32m+[m[32m "serde",[m
 ][m
 [m
 [[package]][m
[31m-name = "quote"[m
[31m-version = "1.0.28"[m
[32m+[m[32mname = "toml_edit"[m
[32m+[m[32mversion = "0.19.14"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "1b9ab9c7eadfd8df19006f1cf1a4aed13540ed5cbc047010ece5826e10825488"[m
[32m+[m[32mchecksum = "f8123f27e969974a3dfba720fdb560be359f57b44302d280ba72e76a74480e8a"[m
 dependencies = [[m
[31m- "proc-macro2",[m
[32m+[m[32m "indexmap 2.0.0",[m
[32m+[m[32m "serde",[m
[32m+[m[32m "serde_spanned",[m
[32m+[m[32m "toml_datetime",[m
[32m+[m[32m "winnow",[m
 ][m
 [m
 [[package]][m
[31m-name = "redox_syscall"[m
[31m-version = "0.3.5"[m
[32m+[m[32mname = "tower"[m
[32m+[m[32mversion = "0.4.13"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "567664f262709473930a4bf9e51bf2ebf3348f2e748ccc50dea20646858f8f29"[m
[32m+[m[32mchecksum = "b8fa9be0de6cf49e536ce1851f987bd21a43b771b09473c3549a6c853db37c1c"[m
 dependencies = [[m
[31m- "bitflags",[m
[32m+[m[32m "futures-core",[m
[32m+[m[32m "futures-util",[m
[32m+[m[32m "pin-project",[m
[32m+[m[32m "pin-project-lite",[m
[32m+[m[32m "tokio",[m
[32m+[m[32m "tower-layer",[m
[32m+[m[32m "tower-service",[m
[32m+[m[32m "tracing",[m
 ][m
 [m
 [[package]][m
[31m-name = "regex"[m
[31m-version = "1.8.4"[m
[32m+[m[32mname = "tower-http"[m
[32m+[m[32mversion = "0.3.5"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "d0ab3ca65655bb1e41f2a8c8cd662eb4fb035e67c3f78da1d61dffe89d07300f"[m
[32m+[m[32mchecksum = "f873044bf02dd1e8239e9c1293ea39dad76dc594ec16185d0a1bf31d8dc8d858"[m
 dependencies = [[m
[31m- "aho-corasick",[m
[31m- "memchr",[m
[31m- "regex-syntax",[m
[32m+[m[32m "bitflags",[m
[32m+[m[32m "bytes",[m
[32m+[m[32m "futures-core",[m
[32m+[m[32m "futures-util",[m
[32m+[m[32m "http",[m
[32m+[m[32m "http-body",[m
[32m+[m[32m "http-range-header",[m
[32m+[m[32m "httpdate",[m
[32m+[m[32m "mime",[m
[32m+[m[32m "mime_guess",[m
[32m+[m[32m "percent-encoding",[m
[32m+[m[32m "pin-project-lite",[m
[32m+[m[32m "tokio",[m
[32m+[m[32m "tokio-util",[m
[32m+[m[32m "tower-layer",[m
[32m+[m[32m "tower-service",[m
 ][m
 [m
 [[package]][m
[31m-name = "regex-syntax"[m
[31m-version = "0.7.2"[m
[32m+[m[32mname = "tower-layer"[m
[32m+[m[32mversion = "0.3.2"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "436b050e76ed2903236f032a59761c1eb99e1b0aead2c257922771dab1fc8c78"[m
[32m+[m[32mchecksum = "c20c8dbed6283a09604c3e69b4b7eeb54e298b8a600d4d5ecb5ad39de609f1d0"[m
 [m
 [[package]][m
[31m-name = "rustix"[m
[31m-version = "0.37.19"[m
[32m+[m[32mname = "tower-service"[m
[32m+[m[32mversion = "0.3.2"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "acf8729d8542766f1b2cf77eb034d52f40d375bb8b615d0b147089946e16613d"[m
[32m+[m[32mchecksum = "b6bc1c9ce2b5135ac7f93c72918fc37feb872bdc6a5533a8b85eb4b86bfdae52"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "tracing"[m
[32m+[m[32mversion = "0.1.37"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "8ce8c33a8d48bd45d624a6e523445fd21ec13d3653cd51f681abf67418f54eb8"[m
 dependencies = [[m
[31m- "bitflags",[m
[31m- "errno",[m
[31m- "io-lifetimes",[m
[31m- "libc",[m
[31m- "linux-raw-sys",[m
[31m- "windows-sys 0.48.0",[m
[32m+[m[32m "cfg-if",[m
[32m+[m[32m "log",[m
[32m+[m[32m "pin-project-lite",[m
[32m+[m[32m "tracing-core",[m
 ][m
 [m
 [[package]][m
[31m-name = "serde"[m
[31m-version = "1.0.164"[m
[32m+[m[32mname = "tracing-core"[m
[32m+[m[32mversion = "0.1.31"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "9e8c8cf938e98f769bc164923b06dce91cea1751522f46f8466461af04c9027d"[m
[32m+[m[32mchecksum = "0955b8137a1df6f1a2e9a37d8a6656291ff0297c1a97c24e0d8425fe2312f79a"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "once_cell",[m
[32m+[m[32m][m
 [m
 [[package]][m
[31m-name = "shell-words"[m
[31m-version = "1.1.0"[m
[32m+[m[32mname = "try-lock"[m
[32m+[m[32mversion = "0.2.4"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "24188a676b6ae68c3b2cb3a01be17fbf7240ce009799bb56d5b1409051e78fde"[m
[32m+[m[32mchecksum = "3528ecfd12c466c6f163363caf2d02a71161dd5e1cc6ae7b34207ea2d42d81ed"[m
 [m
 [[package]][m
[31m-name = "strsim"[m
[31m-version = "0.10.0"[m
[32m+[m[32mname = "typenum"[m
[32m+[m[32mversion = "1.16.0"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "73473c0e59e6d5812c5dfe2a064a6444949f089e20eec9a2e5506596494e4623"[m
[32m+[m[32mchecksum = "497961ef93d974e23eb6f433eb5fe1b7930b659f06d12dec6fc44a8f554c0bba"[m
 [m
 [[package]][m
[31m-name = "syn"[m
[31m-version = "2.0.18"[m
[32m+[m[32mname = "unicase"[m
[32m+[m[32mversion = "2.6.0"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "32d41677bcbe24c20c52e7c70b0d8db04134c5d1066bf98662e2871ad200ea3e"[m
[32m+[m[32mchecksum = "50f37be617794602aabbeee0be4f259dc1778fabe05e2d67ee8f79326d5cb4f6"[m
 dependencies = [[m
[31m- "proc-macro2",[m
[31m- "quote",[m
[31m- "unicode-ident",[m
[32m+[m[32m "version_check",[m
 ][m
 [m
 [[package]][m
[31m-name = "tempfile"[m
[31m-version = "3.6.0"[m
[32m+[m[32mname = "unicode-bidi"[m
[32m+[m[32mversion = "0.3.13"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m-checksum = "31c0432476357e58790aaa47a8efb0c5138f137343f3b5f23bd36a27e3b0a6d6"[m
[31m-dependencies = [[m
[31m- "autocfg",[m
[31m- "cfg-if",[m
[31m- "fastrand",[m
[31m- "redox_syscall",[m
[31m- "rustix",[m
[31m- "windows-sys 0.48.0",[m
[31m-][m
[32m+[m[32mchecksum = "92888ba5573ff080736b3648696b70cafad7d250551175acbaa4e0385b3e1460"[m
 [m
 [[package]][m
 name = "unicode-ident"[m
[36m@@ -467,6 +1853,15 @@[m [mversion = "1.0.9"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
 checksum = "b15811caf2415fb889178633e7724bad2509101cde276048e013b9def5e51fa0"[m
 [m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "unicode-normalization"[m
[32m+[m[32mversion = "0.1.22"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "5c5713f0fc4b5db668a2ac63cdb7bb4469d8c9fed047b1d0292cc7b0ce2ba921"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "tinyvec",[m
[32m+[m[32m][m
[32m+[m
 [[package]][m
 name = "unicode-width"[m
 version = "0.1.10"[m
[36m@@ -484,12 +1879,53 @@[m [mdependencies = [[m
  "unicode-width",[m
 ][m
 [m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "url"[m
[32m+[m[32mversion = "2.4.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "50bff7831e19200a85b17131d085c25d7811bc4e186efdaf54bbd132994a88cb"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "form_urlencoded",[m
[32m+[m[32m "idna",[m
[32m+[m[32m "percent-encoding",[m
[32m+[m[32m][m
[32m+[m
 [[package]][m
 name = "utf8parse"[m
 version = "0.2.1"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
 checksum = "711b9620af191e0cdc7468a8d14e709c3dcdb115b36f838e601583af800a370a"[m
 [m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "uuid"[m
[32m+[m[32mversion = "1.4.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "79daa5ed5740825c40b389c5e50312b9c86df53fccd33f281df655642b43869d"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "getrandom",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "version_check"[m
[32m+[m[32mversion = "0.9.4"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "49874b5167b65d7193b8aba1567f5c7d93d001cafc34600cee003eda787e483f"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "want"[m
[32m+[m[32mversion = "0.3.1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "bfa7760aed19e106de2c7c0b581b509f2f25d3dacaf737cb82ac61bc6d760b0e"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "try-lock",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "wasi"[m
[32m+[m[32mversion = "0.11.0+wasi-snapshot-preview1"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "9c8d87e72b64a3b4db28d11ce29237c246188f4f51057d65a7eab63b7987e423"[m
[32m+[m
 [[package]][m
 name = "winapi"[m
 version = "0.3.9"[m
[36m@@ -644,8 +2080,47 @@[m [mversion = "0.48.0"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
 checksum = "1a515f5799fe4961cb532f983ce2b23082366b898e52ffbce459c86f67c8378a"[m
 [m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "winnow"[m
[32m+[m[32mversion = "0.5.0"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "81fac9742fd1ad1bd9643b991319f72dd031016d44b77039a26977eb667141e7"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "memchr",[m
[32m+[m[32m][m
[32m+[m
 [[package]][m
 name = "zeroize"[m
 version = "1.6.0"[m
 source = "registry+https://github.com/rust-lang/crates.io-index"[m
 checksum = "2a0956f1ba7c7909bfb66c2e9e4124ab6f6482560f6628b5aaeba39207c9aad9"[m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "zstd"[m
[32m+[m[32mversion = "0.12.4"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "1a27595e173641171fc74a1232b7b1c7a7cb6e18222c11e9dfb9888fa424c53c"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "zstd-safe",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "zstd-safe"[m
[32m+[m[32mversion = "6.0.6"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "ee98ffd0b48ee95e6c5168188e44a54550b1564d9d530ee21d5f0eaed1069581"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "libc",[m
[32m+[m[32m "zstd-sys",[m
[32m+[m[32m][m
[32m+[m
[32m+[m[32m[[package]][m
[32m+[m[32mname = "zstd-sys"[m
[32m+[m[32mversion = "2.0.8+zstd.1.5.5"[m
[32m+[m[32msource = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+[m[32mchecksum = "5556e6ee25d32df2586c098bbfa278803692a20d0ab9565e049480d52707ec8c"[m
[32m+[m[32mdependencies = [[m
[32m+[m[32m "cc",[m
[32m+[m[32m "libc",[m
[32m+[m[32m "pkg-config",[m
[32m+[m[32m][m
[1mdiff --git a/Cargo.toml b/Cargo.toml[m
[1mindex dfd02c4..1dd6aec 100644[m
[1m--- a/Cargo.toml[m
[1m+++ b/Cargo.toml[m
[36m@@ -11,11 +11,16 @@[m [mauthors = ["XYZscratcher <xyzscer@outlook.com>"][m
 [m
 [dependencies][m
 anyhow = "1.0.71"[m
[31m-#actix-files = "0.6.2"[m
[31m-#actix-web = "4.3.1"[m
[32m+[m[32maxum = "0.6.19"[m
[32m+[m[32maxum_static = "1.2.2"[m
[32m+[m[32mactix-files = "0.6.2"[m
[32m+[m[32mactix-web = "4"[m
 clap = { version = "4.3.3", features = ["derive", "cargo"] }[m
 colored = "2.0.0"[m
 dialoguer = "0.10.4"[m
[32m+[m[32mfile-serve = "0.2.4"[m
[32m+[m[32mhuman-panic = "1.1.5"[m
 indicatif = "0.17.5"[m
 markdown = "0.3.0"[m
[32m+[m[32mtokio = { version = "1.29.1", features = ["full"] }[m
 upon = "0.6.0"[m
[1mdiff --git a/src/main.rs b/src/main.rs[m
[1mindex 8eb2c37..e6d7c60 100644[m
[1m--- a/src/main.rs[m
[1m+++ b/src/main.rs[m
[36m@@ -1,13 +1,20 @@[m
 //TODO:添加 `lib.rs` ，将项目的 `main.rs` 中的一些逻辑分离到此中[m
[31m-[m
[32m+[m[32muse axum::{[m
[32m+[m[32m    routing::{get, post},[m
[32m+[m[32m    http::StatusCode,[m
[32m+[m[32m    response::IntoResponse,[m
[32m+[m[32m    Json, Router,[m
[32m+[m[32m};[m
[32m+[m[32muse std::net::SocketAddr;[m
 //use actix_files::Files;[m
 //use actix_web::App;[m
 use clap::*;[m
 //use indicatif::*;[m
 use anyhow::Result;[m
[31m-//use colored::Colorize;[m
[31m-use markdown::file_to_html;[m
[31m-//use std::fmt::Display;[m
[32m+[m[32muse colored::Colorize;[m
[32m+[m[32muse human_panic::setup_panic;[m
[32m+[m[32muse markdown::to_html;[m
[32m+[m[32muse std::fmt::Display;[m
 use std::{fs, io::Write, process};[m
 //use upon; 有用到 upon，但没必要将它引入作用域；另见 https://rust-lang.github.io/rust-clippy/master/index.html#/single_component_path_imports[m
 [m
[36m@@ -20,31 +27,26 @@[m [mstruct Page {[m
     name: String,[m
     content: String,[m
 }[m
[31m-/*#[derive(Debug)][m
[32m+[m
[32m+[m[32m#[derive(Debug)][m
 struct AppError {[m
[31m-    kind: AppErrorKind,[m
[32m+[m[32m    kind: String,[m
     msg: String,[m
 }[m
[31m-#[derive(Debug)][m
[31m-enum AppErrorKind {[m
[31m-    Other,[m
[31m-}[m
[32m+[m
 impl Display for AppError {[m
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {[m
[31m-        let err = match self.kind {[m
[31m-            AppErrorKind::Other => "Other",[m
[31m-            _ => "App",[m
[31m-        };[m
[32m+[m[32m        let err = &self.kind;[m
         let err_text = format!("{} Error:", err).red();[m
         write!(f, "{} {}", err_text, self.msg)[m
     }[m
 }[m
 impl std::error::Error for AppError {}[m
 impl AppError {[m
[31m-    fn new(kind: AppErrorKind, msg: String) -> Self {[m
[32m+[m[32m    fn new(kind: String, msg: String) -> Self {[m
         AppError { kind, msg }[m
     }[m
[31m-}*///FIXME:`AppError` 目前好像没用？先注释掉[m
[32m+[m[32m} //FIXME:`AppError` 目前好像没用？先注释掉[m
 #[warn(dead_code)][m
 fn parse_config() {}[m
 fn new(post_name: &str) -> Result<()> {[m
[36m@@ -138,22 +140,49 @@[m [mfn parse() -> Result<Vec<Page>> {[m
     }*/[m
     for post in posts {[m
         let p = post?;[m
[31m-        if p.file_name()!="_index.md" {[m
[31m-        let html = file_to_html(&p.path())?;[m
[31m-        let post_template = engine[m
[31m-            .compile(fs::read_to_string(theme_dir.clone() + "/post.html")?.as_str())?[m
[31m-            .render(upon::value! {content: html})?;[m
[31m-        let name = p[m
[31m-            .file_name()[m
[31m-            .to_str()[m
[31m-            .unwrap()[m
[31m-            .to_string()[m
[31m-            .replacen(".md", ".html", 1);[m
[31m-        dbg!(&name);[m
[31m-        parsed_pages.push(Page {[m
[31m-            name,[m
[31m-            content: post_template,[m
[31m-        })}[m
[32m+[m[32m        if p.file_name() != "_index.md" {[m
[32m+[m[32m            let file = fs::read_to_string(&p.path())?;[m
[32m+[m[32m            let mut flag = file.lines().collect::<Vec<_>>()[0];[m
[32m+[m[32m            enum FrontMatterType {[m
[32m+[m[32m                YAML,[m
[32m+[m[32m                TOML,[m
[32m+[m[32m                None,[m
[32m+[m[32m            }[m
[32m+[m[32m            let mut front_matter_type = FrontMatterType::None; //TODO:解析 Front matter[m
[32m+[m[32m            let [front_matter, other_content] = {[m
[32m+[m[32m                //FIXME:Ahhhhhhhhhhhhhhhhhhhh![m
[32m+[m[32m                //let flag = flag.unwrap();[m
[32m+[m[32m                let fmtype = file.split(flag);[m
[32m+[m[32m                if flag == "---" {[m
[32m+[m[32m                    front_matter_type = FrontMatterType::YAML;[m
[32m+[m[32m                } else if flag == "+++" {[m
[32m+[m[32m                    front_matter_type = FrontMatterType::TOML;[m
[32m+[m[32m                } else {[m
[32m+[m[32m                }[m
[32m+[m[32m                let r = &fmtype.collect::<Vec<_>>()[..];[m
[32m+[m[32m                dbg!(&r);[m
[32m+[m[32m                if r.len() <= 2 {[m
[32m+[m[32m                    [r[1], ""][m
[32m+[m[32m                } else {[m
[32m+[m[32m                    [r[1], r[1]][m
[32m+[m[32m                }[m
[32m+[m[32m            };[m
[32m+[m[32m            let html = to_html(other_content);[m
[32m+[m[32m            let post_template = engine[m
[32m+[m[32m                .compile(fs::read_to_string(theme_dir.clone() + "/post.html")?.as_str())?[m
[32m+[m[32m                .render(upon::value! {content: html})?;[m
[32m+[m[32m            let name = p[m
[32m+[m[32m                .file_name()[m
[32m+[m[32m                .to_str()[m
[32m+[m[32m                .unwrap()[m
[32m+[m[32m                .to_string()[m
[32m+[m[32m                .replacen(".md", ".html", 1);[m
[32m+[m[32m            dbg!(&name);[m
[32m+[m[32m            parsed_pages.push(Page {[m
[32m+[m[32m                name,[m
[32m+[m[32m                content: post_template,[m
[32m+[m[32m            })[m
[32m+[m[32m        }[m
     }[m
     //let template = engine.get_template("index.html").unwrap();[m
     //let result = template.render(upon::value! { user: { name: "John Smith" }})?;[m
[36m@@ -161,12 +190,31 @@[m [mfn parse() -> Result<Vec<Page>> {[m
     dbg!(&parsed_pages);[m
     Ok(parsed_pages)[m
 }[m
[31m-fn server() -> Result<()> {[m
[31m-    //TODO:运行一个静态服务器[m
[31m-    //let app = App::new().service(Files::new("/build", ".").prefer_utf8(true));[m
[32m+[m[32masync fn server() -> Result<()> {[m
[32m+[m[32m    /*let path = std::env::current_dir()?.join("build/");[m
[32m+[m[32m    let server = file_serve::ServerBuilder::new(&path).hostname("localhost").port(4096).build();[m
[32m+[m[32m    println!("Serving {}", path.display());[m
[32m+[m[32m    println!("See http://{}", server.addr());[m
[32m+[m[32m    println!("Hit CTRL-C to stop");[m
[32m+[m[32m    server.serve()?;*/[m
[32m+[m[32m    /*async fn index()->&'static str{[m
[32m+[m[32m        "hello"[m
[32m+[m[32m    }*/[m
[32m+[m[32m    let app = axum_static::static_router("build/");[m
[32m+[m[32m    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));[m
[32m+[m[32m    //tracing::debug!("listening on {}", addr);[m
[32m+[m[32m    println!("http://{addr}");[m
[32m+[m[32m    axum::Server::bind(&addr)[m
[32m+[m[32m        .serve(app.into_make_service())[m
[32m+[m[32m        .await[m
[32m+[m[32m        .unwrap();[m
     Ok(())[m
[32m+[m[32m    //todo!()[m
[32m+[m[32m    //Err(AppError::new("No".to_string(), "test".to_string()).into())[m
 }[m
[31m-fn main() {[m
[32m+[m[32m#[tokio::main][m
[32m+[m[32masync fn main() {[m
[32m+[m[32m    setup_panic!();[m
     /*let ten_millis = time::Duration::from_millis(1);[m
     let now = time::Instant::now();*/[m
     /*std::panic::set_hook(Box::new(|i| {[m
[36m@@ -196,7 +244,7 @@[m [mfn main() {[m
     match matches.subcommand() {[m
         Some(("init", m)) => init(m.get_one::<String>("dir_name").map(|x| x.as_str())),[m
         Some(("build", _)) => build(),[m
[31m-        Some(("server", _)) => server(),[m
[32m+[m[32m        Some(("server", _)) => server().await,[m
         Some(("new", m)) => new(m.get_one::<String>("post_name").unwrap()),[m
         None => {[m
             println!("{}", help_text);[m
[36m@@ -204,10 +252,11 @@[m [mfn main() {[m
         }[m
         _ => Ok(()),[m
     }[m
[31m-    .unwrap_or_else(|e| {[m
[32m+[m[32m    /*.unwrap_or_else(|e| {[m
         eprintln!("{}", e);[m
         process::exit(1)[m
[31m-    });[m
[32m+[m[32m    }); */[m
[32m+[m[32m    .unwrap();[m
     /*let bar = ProgressBar::new(1000);[m
     bar.set_style([m
         ProgressStyle::with_template([m
