## 恒星

恒星到星系中心距离的概率分布函数(PDF)

$$
p(r) = 1 - r^{10}, r \in [0, 1]
$$

恒星质量的概率分布函数(PDF)

$$
p(x) = -(1-x)^{400} + (1-x)^{30} + \frac{\sqrt{x}-x}{50}, x \in [0, 1]\\
$$

其他属性的分布通过插值得到

```rust
let spectral_type = mass_to_st[mass];
let prop_min = st_to_props[spectral_type - 1].prop;
let prop_max = st_to_props[spectral_type + 1].prop;
let prop = prop_min + (prop_max - prop_min) * rand::random::<f64>();
```

### 单位
见[`units_info.json`](../cosmos/assets/config/units_info.json)

1M☉ = 1.9891 * 10^30 kg
1R☉ = 6.957 * 10^8 m
1L☉ = 3.828 * 10^26 W

### 属性

见[`star_properties.json`](../cosmos/assets/config/star_properties.json)
