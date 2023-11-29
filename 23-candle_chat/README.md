## Rust Candle Demo

An interactive command line tool to demonstrate how to use HuggingFace's rust [Candle ML framework](https://github.com/huggingface/candle) to execute LLM. 

This demo uses the quantized version of LLM openchat: https://huggingface.co/TheBloke/openchat_3.5-GGUF by default.


### Prepare

Make sure you have installed the huggingface cli, if not, do it:

```
pip install -U "huggingface_hub[cli]"
```

And then you should download this model file associated with the original openchat `tokenizer.json` file: 


```
mkdir hf_hub
HF_HUB_ENABLE_HF_TRANSFER=1 HF_ENDPOINT=https://hf-mirror.com huggingface-cli download TheBloke/openchat_3.5-GGUF openchat_3.5.Q8_0.gguf  --local-dir hf_hub
HF_HUB_ENABLE_HF_TRANSFER=1 HF_ENDPOINT=https://hf-mirror.com huggingface-cli download openchat/openchat_3.5 tokenizer.json --local-dir hf_hub
```

### Run

There are two examples here:

- **simple**: all parameters are hardcoded into code to make everything simplest, but you need to modify the model and tokenizer.json file by yourself, and run by:

```
cargo run --release --bin simple
```

- **cli**: you can use this cli program to pass parameters from command line. 

```
cargo run --release --bin cli -- --model=xxxxxxx --tokenizer=xxxx
```

You can use `--help` to show what parameters could be configured.

```
$ cargo run --release --bin cli -- --help
    Finished release [optimized] target(s) in 0.04s
     Running `target/release/cli --help`
avx: false, neon: false, simd128: false, f16c: false
Usage: cli [OPTIONS]

Options:
      --tokenizer <TOKENIZER>            [default: ../hf_hub/openchat_3.5_tokenizer.json]
      --model <MODEL>                    [default: ../hf_hub/openchat_3.5.Q8_0.gguf]
  -n, --sample-len <SAMPLE_LEN>          [default: 1000]
      --temperature <TEMPERATURE>        [default: 0.8]
      --seed <SEED>                      [default: 299792458]
      --repeat-penalty <REPEAT_PENALTY>  [default: 1.1]
      --repeat-last-n <REPEAT_LAST_N>    [default: 64]
      --gqa <GQA>                        [default: 8]
  -h, --help                             Print help
  -V, --version                          Print version
```

### License

None.

### Feedback

Feel free to submit issues to this repository.
