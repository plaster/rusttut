fn main() {
	println!("Hello, WebAssempbly!");
}
/*
~/opt/emsdk-portable% ./emsdk install latest
% source ~/opt/emsdk-portable/emsdk_env.sh
% rustc --target=wasm32-unknown-emscripten wasm-hello.rs -o wasm-hello.html
error: could not exec the linker `emcc`: No such file or directory (os error 2)
  |
  = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=0" "-L" "/home/plaster/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib" "wasm-hello.0.o" "-o" "wasm-hello.html" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"___rdl_shrink_in_place\",\"___rdl_alloc_excess\",\"___rdl_usable_size\",\"___rdl_alloc\",\"___rdl_realloc_excess\",\"___rdl_realloc\",\"___rdl_oom\",\"___rdl_grow_in_place\",\"___rdl_alloc_zeroed\",\"___rdl_dealloc\",\"_rust_eh_personality\"]" "wasm-hello.crate.allocator.o" "-O0" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/home/plaster/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib" "/home/plaster/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-881b9fdbdf1d515b.rlib" "/home/plaster/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc_system-3c10208cdd7e61cb.rlib" "/home/plaster/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/librand-32f648f7f7567c6c.rlib" "/home/plaster/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-9cbadc6554202be9.rlib" "/home/plaster/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-bc70b4efeaeb398c.rlib" "/home/plaster/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_unicode-a0ad42dc8f5856aa.rlib" "/home/plaster/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-618c266cf9124966.rlib" "/home/plaster/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-16f3b02b9a976b94.rlib" "/home/plaster/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-21491ce3d14f1ef2.rlib" "/home/plaster/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-b9713bd7f605c0b2.rlib" "-l" "c" "-s" "BINARYEN=1" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1"

error: aborting due to previous error
*/
