# rust_waraiotoko_demo

方々で実装されている笑い男のヤツをrustで実装してみた

## ビルド時の注意

openCVの入れ方は、下記を参照

[GitHub - twistedfall/opencv-rust: Rust bindings for OpenCV 3](https://github.com/twistedfall/opencv-rust)

`cargo biuid`した後は、手動でopencv_world460.dll等の必要ファイルを手動でコピーする必要あり。

## 使い方

下記3種類のキーでプログラムを制御します。

i：顔検出開始

0：顔検出中断

q：プログラム終了



## 参考情報

[opencv - Rust](https://docs.rs/opencv/latest/opencv/index.html)

[GitHub - sunsided/rust-facedetect: OpenCV Face Detection in Rust - a test project](https://github.com/sunsided/rust-facedetect)
