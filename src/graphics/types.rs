use gfx;

pub type WindowSettings<'a> = (&'a str, u32, u32);
pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;
