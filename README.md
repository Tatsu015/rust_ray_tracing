# cpp_ray_tracing

[Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

[週末レイトレーシング](https://inzkyk.xyz/ray_tracing_in_one_weekend/)

Generate ray tracing image by Rust

![out](https://user-images.githubusercontent.com/97239922/243361825-88d7c230-fb02-442c-92e1-0007ceefb987.png)

## Usage

### build

Execute the following command.

~~~bash
cargo build --release
~~~

if you want to use debug infomation, use following command.

~~~bash
cargo build
~~~

but debug mode is very slow compared to release mode.

### export image

Execute the following command, and generate file to ./out.ppm

~~~bash
./target/release/rust_ray_tracing > out.ppm
~~~
