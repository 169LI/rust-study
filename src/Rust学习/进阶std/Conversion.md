# Conversion

[std::convert - Rust](https://doc.rust-lang.org/std/convert/index.html)


本模块中的特性提供了一种从一种类型转换为另一种类型的方法。每个特性都有不同的用途：

* 为廉价的引用到引用的转换实现 AsRef 特性
* 为廉价的可变到可变的转换实现 AsMut 特性
* 为消耗性的值到值的转换实现 From 特性
* 实现 Into 特性以消耗当前 crate 外部的值到值转换
* TryFrom 和 TryInto 特性类似于 From 和 Into ，但应在转换可能失败时实现。
