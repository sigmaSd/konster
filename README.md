# konster

This crate exposes const equivalents of standard library types that are usable on stable.

```rust
   use konster::kstr::KStr;

   const _: () = {
       let mut str = KStr::<20>::new();
       str = str.push(4);
       let (str, val) = match str.pop() {
            Some((str,val)) => (str, val),
            _ => unreachable!(),
       };
       if !str.is_empty() {
            panic!("Str is not empty");
       }
   };
```
