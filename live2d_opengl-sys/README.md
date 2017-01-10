# live2d_opengl-sys
Live2D Cubism SDK OpenGL ラッパー（今のところ MSVC だけ）

# ビルド方法
1. 環境変数 `LIVE2DOPENGL_INCLUDE_DIR` に SDK の include ディレクトリを設定
2. 環境変数 `LIVE2DOPENGL_LIB_DIR` に live2d_opengl.lib があるディレクトリを設定
    - アーキテクチャとか VS のバージョンとかにあわせてうまいことやってください
    - rustc は --release を指定しても link.exe をデバッグモードで呼び出すので、 Debug 版の lib を使用してください
3. `cargo build`
