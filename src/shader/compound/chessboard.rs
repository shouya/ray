

struct Chessboard {
  shaders: (ShaderType, ShaderType),
  mapper: Box<Fn (V3) -> V2>
}