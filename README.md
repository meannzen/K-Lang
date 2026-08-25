# K-Lang

K-Lang is a custom programming language interpreter built from scratch. Designed with a Unicode-aware tokenizer, K-Lang brings native Khmer script.


## Example
```
// A simple conditional example in K-Lang
តាង ប្រាក់ខែ = ៥00;

បើ (ប្រាក់ខែ > ៤00) {
    បោះពុម្ព("ត្រូវបង់ពន្ធ");
} បើពុំនោះទេ {
    បោះពុម្ព("មិនបាច់បង់ពន្ធ");
}
```

## Running script

```bash
./bin/klang examples/hello.k
```
## Contribution
K-Lang is an independent interpreter development project. Feedback, bug reports, and discussions on Khmer computational linguistics are very welcome!

## License
This project is open-source under the MIT License.
