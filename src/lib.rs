use anyhow::Result; // Automatically handle the error types
use opencv::core::{Rect, Scalar, Size};
use opencv::{
    prelude::*,
    imgproc,
    objdetect,
    types
}; 

//type Result<T> = opencv::Result<T>;

// private関数
// グレースケール化
fn convert_to_grayscale(src: &Mat) -> Result<Mat> {
    let mut gray = Mat::default();
    imgproc::cvt_color(src, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
    Ok(gray)
}

// ヒストグラム平坦化
fn equalize_image(src: &Mat) -> Result<Mat> {
    let mut equalized = Mat::default();
    imgproc::equalize_hist(src, &mut equalized)?;
    Ok(equalized)
}

// pubulic関数
// グレースケールからのヒストグラム平坦化
pub fn preprocess_image(frame: &Mat) -> Result<Mat> {
    let gray = convert_to_grayscale(frame)?;
    let equalized = equalize_image(&gray)?;
    Ok(equalized)
}

// 顔検出
pub fn detect_faces(
    classifier: &mut objdetect::CascadeClassifier,
    image: Mat,
) -> Result<types::VectorOfRect> {
    const SCALE_FACTOR: f64 = 1.1;
    const MIN_NEIGHBORS: i32 = 2;
    const FLAGS: i32 = 0;
    const MIN_FACE_SIZE: Size = Size {
        width: 50,
        height: 50,
    };
    const MAX_FACE_SIZE: Size = Size {
        width: 0,
        height: 0,
    };

    let mut faces = types::VectorOfRect::new();
    classifier.detect_multi_scale(
        &image,
        &mut faces,
        SCALE_FACTOR,
        MIN_NEIGHBORS,
        FLAGS,
        MIN_FACE_SIZE,
        MAX_FACE_SIZE,
    )?;
    Ok(faces)
}

pub fn intercept(frame: &mut Mat, waraiotoko: &mut Mat, face: Rect) -> Result<()> {
    println!("found face {:?}", face);
    let FACE_AREA = Rect {
        x: face.x ,
        y: face.y,
        width: face.width,
        height: face.height,
    };

    let FACE_SIZE = Size {
        width: face.width,
        height: face.height,
    };

    // 例のロゴをリサイズして前景画像とする
    let mut fg = Mat::default();
    imgproc::resize(waraiotoko, &mut fg, FACE_SIZE, 0.0, 0.0, imgproc::INTER_LINEAR)?;

    // 背景画像から顔のある部分を切り出す
    let mut bg_roi = Mat::roi(frame, FACE_AREA)?;

    // 背景色を無視するためのマスクを生成する
    // カラー画像⇒グレースケース化⇒2値化⇒24bitカラーに戻すのステップで作成する
    let mut fg_gray = Mat::default();
    let mut fg_bin = Mat::default();
    let mut fg_bin_24bit = Mat::default();
    fg_gray = convert_to_grayscale(&fg)?;
    imgproc::threshold(&fg_gray, &mut fg_bin, 1.0, 255.0, imgproc::THRESH_BINARY)?;
    imgproc::cvt_color(&fg_bin, &mut fg_bin_24bit, imgproc::COLOR_GRAY2BGR, 0)?;

    // 背景色を考慮して描画
    fg.copy_to_masked(&mut bg_roi, &fg_bin_24bit)?;

    Ok(())
}