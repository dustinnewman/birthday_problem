# Birthday Problem

The birthday problem is a statistical phenomenon that the number of people in a room with the same birthday is far lower than intuitively expected. For example, the likelihood that two people in a room of 23 share a birthday is around 50%. The results are below:

|  n  | Probability |
|-----|-------------|
| 1   | 0%          |
| 10  | 11.7%       |
| 23  | 50.7%       |
| 30  | 70.6%       |
| 50  | 97%         |
| 70  | 99.9%       |
| 100 | 99.999%     |

I built a Rust program to experimentally validate this. You can run as:

```rust
cargo run <n>
```

If no number is specified, `n` will default to 23.

