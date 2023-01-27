# Starscii
Input ascii characters and get a `*-printed` version.

For example input `c C` to get something like this

```
     *****
     *    
***  *    
*    *    
***  *****
```

## Specs
- Printing is from top to down
- User can specify font-size, font-type, filler etc..
- Spaces are allowed.
- User can pick any character to be used as filler aside `*` ...

### Usage

```rust
    fn main() {
    let config = StarsciiConfig {
        font_size: 99,
        font_type: Box::new(BoxFont{}),
        filler: '*'
    };
    let starscii = Starscii::new(config);

    println!("{}", starscii.draw("c"));
}
```