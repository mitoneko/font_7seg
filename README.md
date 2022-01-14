# font_7seg
embedded_graphicsのTextクラスに対応する7セグメントLED風の数字フォントです。0-9までの数字と小数点(.)に対応します。

# 使用例

```
let font = Font7Seg::new(Size::new(10,20), Rgb565::RED);
Text::new("0123", Point::new(1,1), font).draw(&mut display)?;
```

# ライセンス
そのライブラリは、次のライセンス条件で利用できます。
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) または
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) または http://opensource.org/licenses/MIT)
