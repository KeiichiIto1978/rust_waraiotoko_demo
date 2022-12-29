mod lib;

//pub use crate::lib::Interceptor;

use std::env;
use std::path::Path;

use anyhow::Result; // Automatically handle the error types
use opencv::core::{Rect, Scalar, Size};
use opencv::{
    prelude::*,
    videoio,
    highgui,
    objdetect,
    imgcodecs,
    types
}; // Note, the namespace of OpenCV is changed (to better or worse). It is no longer one enormous.

const CASCADE_XML_FILE: &str = "./haarcascade_frontalface_alt.xml"; // カスケードファイル
const WARAIOTOKO_IMAGE_FILE: &str = "./waraiotoko.bmp"; // 笑い男の顔画像


fn main() -> Result<()> { // Note, this is anyhow::Result

    // カスケードファイルの読み込み
    let mut classifier = objdetect::CascadeClassifier::new(CASCADE_XML_FILE)?;

    // 笑い男のロゴを読み込む
    let mut waraiotoko = imgcodecs::imread(WARAIOTOKO_IMAGE_FILE, imgcodecs::IMREAD_UNCHANGED)?;

    // window title
    let title = "SAC's demo was implemented in RUST.";

    // 表示Window作る
    highgui::named_window(title, highgui::WINDOW_AUTOSIZE)?;
    
    // カメラ映像入力用
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();

    // インターセプトを制御する変数
    let mut mode = false;
    
    // メインループ
    loop {
        cam.read(&mut frame)?;

        if mode == true {
            let preprocess_frame = lib::preprocess_image(&frame)?;
            let face_list = lib::detect_faces(&mut classifier, preprocess_frame)?;
            for face in face_list {
                lib::intercept(&mut frame, &mut waraiotoko, face)?;
            }
        } 
        
        highgui::imshow(title, &frame)?;

        let key = highgui::wait_key(1)?;
        if key == 105 { // iでインターセプトを開始
            mode = true;
        }
        else if key == 111 { // oでインターセプトを中断
            mode = false;
        }       
        if key == 113 { // qで終了
            break;
        }
    }
    Ok(())
}