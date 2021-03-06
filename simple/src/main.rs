extern crate glium;
extern crate image;
extern crate live2d_opengl_sys as l2d;

use std::error::Error;
use std::fmt;
use std::os::raw::{c_float, c_int, c_uint};
use std::path;
use glium::{glutin, DisplayBuild, GlObject, Surface};

fn main() {
    const INITIAL_WIDTH: u32 = 600;
    const INITIAL_HEIGHT: u32 = 600;

    let sample_dir = {
        // ヘッダーディレクトリの 1 個上とかいうクソみたいな判定
        match std::env::var_os("LIVE2DOPENGL_INCLUDE_DIR") {
            Some(include_dir) => {
                let mut pb = path::PathBuf::from(include_dir);
                if !pb.pop() {
                    panic!("couldn't find the SDK directory");
                }
                pb.push("sample/simple/proj.win32/simple");
                pb
            },
            None => panic!("LIVE2DOPENGL_INCLUDE_DIR is not set")
        }
    };

    let display = glutin::WindowBuilder::new()
        .with_dimensions(INITIAL_WIDTH, INITIAL_HEIGHT)
        .build_glium()
        .unwrap();

    unsafe { l2d::init(); }
    let mut live2d_model = load_model(sample_dir.join("haru.moc")).unwrap();

    // TextureAny が Drop を実装しているので、テクスチャが削除されないように保持しておく
    let textures = ["texture_00.png", "texture_01.png", "texture_02.png"]
        .iter()
        .map(|x| load_texture(&display, sample_dir.join(x)).unwrap())
        .collect::<Vec<_>>();
    for (i, x) in textures.iter().enumerate() {
        unsafe { live2d_model.setTexture(i as c_int, x.get_id()); }
    }

    unsafe { live2d_model.setPremultipliedAlpha(false); }
    resize(&mut live2d_model, INITIAL_WIDTH, INITIAL_HEIGHT);

    let param_angle_x = std::ffi::CString::new("PARAM_ANGLE_X").unwrap();

    loop {
        let mut target = display.draw();
        let (w, h) = target.get_dimensions();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        unsafe {
            let t = (l2d::ut_system::getUserTimeMSec() as f64) / 1000.0 * 2.0 * std::f64::consts::PI;
            live2d_model.setParamFloat(&param_angle_x, (30.0 * (t / 3.0).sin()) as c_float, 1.0);
            live2d_model.update();
        }

        unsafe {
            display.exec_in_context(|| {
                const GL_ALL_ATTRIB_BITS: c_uint = 0x000fffff;
                glPushAttrib(GL_ALL_ATTRIB_BITS);

                glViewport(0, 0, w as c_int, h as c_int);
                live2d_model.draw();

                glPopAttrib();
            });
        }

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glutin::Event::Closed => return,
                glutin::Event::Resized(w, h) => resize(&mut live2d_model, w, h),
                _ => ()
            }
        }
    }
}

fn load_model<P: AsRef<path::Path>>(path: P) -> Result<l2d::Live2DModelWinGL, ()> {
    // FromFile を使うと日本語パスがあったら死にそうな予感がするから、一度メモリにロード
    let buf = {
        let mut f = std::fs::File::open(path).map_err(|_| ())?;
        let mut buf = Vec::new();
        std::io::Read::read_to_end(&mut f, &mut buf).map_err(|_| ())?;
        buf
    };

    let model = unsafe { l2d::Live2DModelWinGL::loadModel(&buf) };

    if unsafe { model.getCanvasWidth() == 0.0 || model.getCanvasHeight() == 0.0 } {
        Err(())
    } else {
        Ok(model)
    }
}

#[derive(Debug)]
enum LoadTextureError {
    OpenImage(image::ImageError),
    CreateTexture(glium::texture::TextureCreationError)
}

impl fmt::Display for LoadTextureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            LoadTextureError::OpenImage(ref x) => fmt::Display::fmt(x, f),
            LoadTextureError::CreateTexture(ref x) => fmt::Display::fmt(x, f),
        }
    }
}

impl Error for LoadTextureError {
    fn description(&self) -> &str {
        "load texture failed"
    }

    fn cause(&self) -> Option<&Error> {
        Some(match *self {
            LoadTextureError::OpenImage(ref x) => x,
            LoadTextureError::CreateTexture(ref x) => x,
        })
    }
}

impl From<image::ImageError> for LoadTextureError {
    fn from(e: image::ImageError) -> LoadTextureError {
        LoadTextureError::OpenImage(e)
    }
}

impl From<glium::texture::TextureCreationError> for LoadTextureError {
    fn from(e: glium::texture::TextureCreationError) -> LoadTextureError {
        LoadTextureError::CreateTexture(e)
    }
}

fn load_texture<F, P>(facade: &F, path: P)
    -> Result<glium::texture::SrgbTexture2d, LoadTextureError>
    where F: glium::backend::Facade, P: AsRef<path::Path>
{
    let img = match image::open(path)? {
        image::DynamicImage::ImageRgba8(x) => x,
        x => x.to_rgba(),
    };
    let dim = (img.width(), img.height());
    let data = glium::texture::RawImage2d::from_raw_rgba(img.into_raw(), dim);
    Ok(glium::texture::SrgbTexture2d::new(facade, data)?)
}

fn resize(model: &mut l2d::Live2DModelWinGL, w: u32, h: u32) {
    let aspect = (w as f32) / (h as f32);
    let sx = 2.0 / unsafe { model.getCanvasWidth() };
    let sy = -2.0 / unsafe { model.getCanvasWidth() } * aspect;
    let x = -1.0f32;
    let y = 1.0f32;
    let mut matrix = [
        sx, 0.0, 0.0, 0.0,
        0.0, sy, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        x, y, 0.0, 1.0,
    ];
    unsafe { model.setMatrix(matrix.as_mut_ptr()); }
}

extern "system" {
    fn glViewport(x: c_int, y: c_int, width: c_int, height: c_int);
    fn glPushAttrib(mask: c_uint);
    fn glPopAttrib();
}
