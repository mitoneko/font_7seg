//! これは、embedded_graphics対応の、7セグメントLED風フォントです。
//!
//! 実装する文字は、"0123456789."の11種類です。他の文字を渡すと無視します。
//!
//! Font7Seg::new()で、任意の大きさと色を指定することができます。
//! embedded_graphics::text::CharacterStyleも実装しますが、有効なのは、
//! set_text_colorとset_background_colorのみです。
//! その他は、空の実装を継承しています。
//!
//! embedded_graphics::TextRendererの実装中、BaseLine引数に関しては、未実装です。
//! 何を指定しても、原点は、左上隅となります。
//!
//! # Examples
//!
//! ```
//! # use embedded_graphics::{prelude::*, text::Text, pixelcolor::Rgb565};
//! # use font_7seg::Font7Seg;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # fn try_main() -> Result<(), core::convert::Infallible> {
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//! # display.set_allow_overdraw(true);
//! let font = Font7Seg::new(Size::new(10,20), Rgb565::RED);
//! Text::new("0123", Point::new(1,1), font).draw(&mut display)?;
//! # Ok(())
//! # }
//! # fn main() {
//! #   try_main().unwrap();
//! # }
//! ```
//!
#![no_std]
use embedded_graphics as eg;
use num_traits::float::FloatCore;

use eg::pixelcolor::PixelColor;
use eg::prelude::*;
use eg::primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle};
use eg::text::renderer::{CharacterStyle, TextMetrics, TextRenderer};
use eg::text::Baseline;

/// 7セグメントLED風フォント
#[derive(Debug, Clone, Copy)]
pub struct Font7Seg<C> {
    size: Size,
    text_color: C,
    background_color: Option<C>,
    line_width_rate: f32,
    top_margin_rate: f32,
    left_margin_rate: f32,
    point_width_rate: f32,
}

impl<C: PixelColor> Font7Seg<C> {
    /// フォントオブジェクトを生成します。
    /// * `size`       - 表示する数字のサイズ(ピクセル単位)
    /// * `text_color` - 表示する数字の色
    pub fn new(size: Size, text_color: C) -> Self {
        Self {
            size,
            text_color,
            background_color: None,
            line_width_rate: 0.2,
            top_margin_rate: 0.05,
            left_margin_rate: 0.05,
            point_width_rate: 0.2,
        }
    }

    /// 現在の表示する数字のサイズを返します。
    pub fn character_size(&self) -> Size {
        self.size
    }

    fn draw_segment_vert<D>(&self, area: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        // 長さを少し短くする。
        let Size { width, height } = area.bounding_box().size;
        let height_diff = height as f32 * 0.1;
        let area_top = ((height_diff + width as f32) / 2.0).floor();
        let area_top_left = Point::new(0, area_top as i32);
        let area_height = (height as f32 - height_diff * 2.0).floor();
        let area_size = Size::new(width, area_height as u32);
        let mut area = area.cropped(&Rectangle::new(area_top_left, area_size));

        // 描画
        let Size { width, height } = area.bounding_box().size;
        let w_center: f32 = width as f32 / 2.0;
        let v_base_top: f32 = width as f32 * 1.2 / 2.0;
        let v_base_bottom: f32 = height as f32 - v_base_top;
        let w_center: i32 = w_center as i32;
        let v_base_top: i32 = v_base_top as i32;
        let v_base_bottom: i32 = v_base_bottom as i32;
        let points = [
            Point::new(w_center, 0),
            Point::new(width as i32, v_base_top),
            Point::new(width as i32, v_base_bottom),
            Point::new(w_center, height as i32),
            Point::new(0, v_base_bottom),
            Point::new(0, v_base_top),
        ];
        self.draw_polygon(&points, &mut area)
    }

    fn draw_segment_hori<D>(&self, area: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        // 両端を幅の半分だけ削る
        let Size { width, height } = area.bounding_box().size;
        let half_width: i32 = (height as f32 * 1.2 / 2.0).ceil() as i32;
        let new_top_left = Point::new(half_width, 0);
        let new_size = Size::new(width - (half_width * 2) as u32, height);
        let mut area = area.cropped(&Rectangle::new(new_top_left, new_size));

        //セグメントの描画
        let Size { width, height } = area.bounding_box().size;
        let v_center: f32 = height as f32 / 2.0;
        let h_base_left: f32 = height as f32 * 1.2 / 2.0;
        let h_base_right: f32 = width as f32 - h_base_left;
        let v_center: i32 = v_center as i32;
        let h_base_left: i32 = h_base_left as i32;
        let h_base_right: i32 = h_base_right as i32;
        let points = [
            Point::new(0, v_center),
            Point::new(h_base_left, 0),
            Point::new(h_base_right, 0),
            Point::new(width as i32, v_center),
            Point::new(h_base_right, height as i32),
            Point::new(h_base_left, height as i32),
        ];
        self.draw_polygon(&points, &mut area)
    }

    fn draw_polygon<D>(&self, points: &[Point; 6], area: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(self.text_color)
            .build();
        Triangle::new(points[0], points[1], points[5])
            .into_styled(style)
            .draw(area)?;
        Triangle::new(points[2], points[3], points[4])
            .into_styled(style)
            .draw(area)?;
        Rectangle::with_corners(points[5], points[2])
            .into_styled(style)
            .draw(area)?;
        Ok(())
    }

    fn draw_seg_a<D>(&self, area: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let Size { width, .. } = area.bounding_box().size;
        let seg_height: u32 = (width as f32 * self.line_width_rate).ceil() as u32;
        let seg_top_left = Point::new(0, 0);
        let seg_size = Size::new(width, seg_height);
        let mut seg_target = area.cropped(&Rectangle::new(seg_top_left, seg_size));
        self.draw_segment_hori(&mut seg_target)?;
        Ok(())
    }

    fn draw_seg_b<D>(&self, area: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let Size { width, height } = area.bounding_box().size;
        let seg_width: f32 = width as f32 * self.line_width_rate;
        let seg_width: u32 = seg_width.ceil() as u32;
        let seg_height: u32 = (height as f32 / 2.0).ceil() as u32;
        let seg_top_left = Point::new((width - seg_width) as i32, 0);
        let seg_size = Size::new(seg_width, seg_height);
        let mut seg_target = area.cropped(&Rectangle::new(seg_top_left, seg_size));
        self.draw_segment_vert(&mut seg_target)?;
        Ok(())
    }

    fn draw_seg_c<D>(&self, area: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let Size { width, height } = area.bounding_box().size;
        let seg_width: f32 = width as f32 * self.line_width_rate;
        let seg_width_half: f32 = seg_width / 2.0;
        let seg_width: u32 = seg_width.ceil() as u32;
        let seg_height: u32 = (height as f32 / 2.0).ceil() as u32;
        let seg_top: i32 = (height as f32 / 2.0 - seg_width_half).ceil() as i32;
        let seg_left: i32 = (width - seg_width) as i32;
        let seg_top_left = Point::new(seg_left, seg_top);
        let seg_size = Size::new(seg_width, seg_height);
        let mut seg_target = area.cropped(&Rectangle::new(seg_top_left, seg_size));
        self.draw_segment_vert(&mut seg_target)?;
        Ok(())
    }

    fn draw_seg_d<D>(&self, area: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let Size { width, height } = area.bounding_box().size;
        let seg_height: f32 = width as f32 * self.line_width_rate;
        let seg_top: i32 = (height as f32 - seg_height).floor() as i32;
        let seg_height: u32 = seg_height.ceil() as u32;
        let seg_top_left = Point::new(0, seg_top);
        let seg_size = Size::new(width, seg_height);
        let mut seg_target = area.cropped(&Rectangle::new(seg_top_left, seg_size));
        self.draw_segment_hori(&mut seg_target)?;
        Ok(())
    }

    fn draw_seg_e<D>(&self, area: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let Size { width, height } = area.bounding_box().size;
        let seg_width: f32 = width as f32 * self.line_width_rate;
        let seg_width_half: f32 = seg_width / 2.0;
        let seg_width: u32 = seg_width.ceil() as u32;
        let seg_height: u32 = (height as f32 / 2.0).ceil() as u32;
        let seg_top: i32 = (height as f32 / 2.0 - seg_width_half).ceil() as i32;
        let seg_top_left = Point::new(0, seg_top);
        let seg_size = Size::new(seg_width, seg_height);
        let mut seg_target = area.cropped(&Rectangle::new(seg_top_left, seg_size));
        self.draw_segment_vert(&mut seg_target)?;
        Ok(())
    }

    fn draw_seg_f<D>(&self, area: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let Size { width, height } = area.bounding_box().size;
        let seg_width: f32 = width as f32 * self.line_width_rate;
        let seg_width: u32 = seg_width.ceil() as u32;
        let seg_height: u32 = (height as f32 / 2.0).ceil() as u32;
        let seg_top_left = Point::new(0, 0);
        let seg_size = Size::new(seg_width, seg_height);
        let mut seg_target = area.cropped(&Rectangle::new(seg_top_left, seg_size));
        self.draw_segment_vert(&mut seg_target)?;
        Ok(())
    }

    fn draw_seg_g<D>(&self, area: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let Size { width, height } = area.bounding_box().size;
        let seg_height: f32 = width as f32 * self.line_width_rate;
        let seg_height_half: f32 = seg_height / 2.0;
        let seg_top = (height as f32 / 2.0 - seg_height_half).floor();
        let seg_top = seg_top as i32;
        let seg_height: u32 = seg_height.ceil() as u32;
        let seg_top_left = Point::new(0, seg_top);
        let seg_size = Size::new(width, seg_height);
        let mut seg_target = area.cropped(&Rectangle::new(seg_top_left, seg_size));
        self.draw_segment_hori(&mut seg_target)?;
        Ok(())
    }

    fn draw_seg_point<D>(&self, area: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        // 幅を狭くする
        let Size { width, height } = area.bounding_box().size;
        let n_width: f32 = (width as f32 * self.point_width_rate).ceil();
        let n_size = Size::new(n_width as u32, height);
        let mut area = area.cropped(&Rectangle::new(Point::new(0, 0), n_size));

        // 丸を描画
        let Size { width, height } = area.bounding_box().size;
        let radius = width as f32 / 2.0;
        let center_y = height as f32 - radius * 2.0;
        let center = Point::new(radius.floor() as i32, center_y.floor() as i32);
        let style = PrimitiveStyleBuilder::new()
            .fill_color(self.text_color)
            .build();
        Circle::with_center(center, (radius * 2.0).floor() as u32)
            .into_styled(style)
            .draw(&mut area)?;
        Ok(())
    }

    /// 数字を一文字描画する。
    /// <引数>
    /// * num: 描画する数字一桁。10以上の数値の場合、一桁目のみ有効。
    /// * point: trueの場合、小数点を描画する。numは無視される。
    /// * area: 描画対象のDrawTargetの可変参照
    /// <戻り値>
    /// 正常の場合、描画した幅を返す。DrawTarget.draw()のエラーの可能性あり。
    ///
    fn draw_number<D>(&self, num: u32, point: bool, area: &mut D) -> Result<u32, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        // areaから、マージンを削除して領域を再定義
        let Size { width, height } = area.bounding_box().size;
        let all_area_width = width;
        let top_margin: f32 = height as f32 * self.top_margin_rate;
        let left_margin: f32 = width as f32 * self.left_margin_rate;
        let top_left = Point::new(top_margin.ceil() as i32, left_margin.ceil() as i32);
        let size = Size::new(
            width - left_margin.ceil() as u32 * 2,
            height - top_margin.ceil() as u32 * 2,
        );
        let mut area = area.cropped(&Rectangle::new(top_left, size));
        //　描画
        const SEG_PATS: [u8; 10] = [
            0b0011_1111,
            0b0000_0110,
            0b0101_1011,
            0b0100_1111,
            0b0110_0110,
            0b0110_1101,
            0b0111_1101,
            0b0010_0111,
            0b0111_1111,
            0b0110_1111,
        ];
        if point {
            self.draw_seg_point(&mut area)?;
        } else {
            let seg_pat = SEG_PATS[(num % 10) as usize];
            if seg_pat & 0b0000_0001 != 0 {
                self.draw_seg_a(&mut area)?;
            }
            if seg_pat & 0b0000_0010 != 0 {
                self.draw_seg_b(&mut area)?;
            }
            if seg_pat & 0b0000_0100 != 0 {
                self.draw_seg_c(&mut area)?;
            }
            if seg_pat & 0b0000_1000 != 0 {
                self.draw_seg_d(&mut area)?;
            }
            if seg_pat & 0b0001_0000 != 0 {
                self.draw_seg_e(&mut area)?;
            }
            if seg_pat & 0b0010_0000 != 0 {
                self.draw_seg_f(&mut area)?;
            }
            if seg_pat & 0b0100_0000 != 0 {
                self.draw_seg_g(&mut area)?;
            }
        }

        let draw_width = if point {
            let p_width = (size.width as f32 * self.point_width_rate).ceil() as u32;
            all_area_width - size.width + p_width
        } else {
            all_area_width
        };
        Ok(draw_width)
    }

    fn calc_point_width(&self) -> u32 {
        let left_margin: f32 = self.size.width as f32 * self.left_margin_rate;
        let p_width = (self.size.width as f32 - left_margin * 2.0) * self.point_width_rate;
        (p_width + left_margin * 2.0).ceil() as u32
    }
}

impl<C: PixelColor> CharacterStyle for Font7Seg<C> {
    type Color = C;
    fn set_text_color(&mut self, text_color: Option<Self::Color>) {
        if let Some(color) = text_color {
            self.text_color = color;
        }
    }
    fn set_background_color(&mut self, background_color: Option<Self::Color>) {
        self.background_color = background_color;
    }
}

impl<C: PixelColor> TextRenderer for Font7Seg<C> {
    type Color = C;
    fn draw_string<D>(
        &self,
        text: &str,
        pos: Point,
        _baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let mut cur_pos = pos;
        for c in text.chars() {
            let mut num_target = target.cropped(&Rectangle::new(cur_pos, self.size));
            if let Some(bg_color) = self.background_color {
                num_target.clear(bg_color)?;
            }
            let w = if c.is_ascii_digit() {
                let num = c.to_digit(10).unwrap();
                self.draw_number(num, false, &mut num_target)?
            } else if c == '.' {
                self.draw_number(0, true, &mut num_target)?
            } else {
                0
            };
            cur_pos += Size::new(w, 0);
        }
        Ok(cur_pos)
    }

    fn draw_whitespace<D>(
        &self,
        width: u32,
        pos: Point,
        _baseline: Baseline,
        _target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        Ok(pos + Size::new(self.size.width * width, 0))
    }

    fn measure_string(&self, text: &str, pos: Point, _baseline: Baseline) -> TextMetrics {
        let mut width = 0;
        for c in text.chars() {
            if c.is_ascii_digit() {
                width += self.size.width;
            } else if c == '.' {
                width += self.calc_point_width();
            }
        }
        let bounding_box = Rectangle::new(pos, Size::new(width, self.size.height));
        TextMetrics {
            bounding_box,
            next_position: pos + Size::new(width, 0),
        }
    }

    fn line_height(&self) -> u32 {
        self.size.height
    }
}
