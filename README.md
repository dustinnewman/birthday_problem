# Birthday Problem

The birthday problem is a statistical phenomenon where the chances two people in a room have the same birthday are far higher than intuitively expected. For example, the likelihood that two people in a room of 23 share a birthday is around 50%. Most people would expect (given an even distribution of births) you would need 365 / 2 = 183 people to have a 50-50 chance that two have the same birthday. The actual results are below:

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

```
cargo run <n>
```

If no number is specified, `n` will default to 23.

