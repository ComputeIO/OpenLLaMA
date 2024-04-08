# OpenLLaMA

[![Build Status](https://github.com/computeio/openllama/actions/workflows/rust.yml/badge.svg)](https://github.com/computeio/openllama/actions/workflows/rust.yml)
[![LICENSE](https://img.shields.io/github/license/computeio/openllama.svg)](https://github.com/computeio/openllama/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/Language-Rust-blue.svg)](https://www.rust-lang.org/)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/computeio/openllama)](https://rust-reportcard.xuri.me/badge/github.com/computeio/openllama)
[![CII Best Practices](https://bestpractices.coreinfrastructure.org/projects/2761/badge)](https://bestpractices.coreinfrastructure.org/projects/6232)
[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/computeio/openllama/badge)](https://securityscorecards.dev/viewer/?uri=github.com/computeio/openllama)
[![Codecov](https://img.shields.io/codecov/c/github/computeio/openllama?style=flat-square&logo=codecov)](https://codecov.io/gh/computeio/openllama)
[![CLOMonitor](https://img.shields.io/endpoint?url=https://clomonitor.io/api/projects/cncf/chubao-fs/badge)](https://clomonitor.io/projects/cncf/chubao-fs)
[![Release](https://img.shields.io/github/v/release/computeio/openllama.svg?color=161823&style=flat-square&logo=smartthings)](https://github.com/computeio/openllama/releases)
[![Tag](https://img.shields.io/github/v/tag/computeio/openllama.svg?color=ee8936&logo=fitbit&style=flat-square)](https://github.com/computeio/openllama/tags)
[![Chat](https://img.shields.io/badge/zulip-join_chat-brightgreen.svg)](https://openllama.zulipchat.com/)

## Overview

A Rust LLaMA project to **load**, **serve** and **extend** LLM models.

# Key Objectives

- [ ] Support both [GGML](https://github.com/rustformers/llm/blob/main/crates/ggml/README.md) and [HF](https://github.com/ggerganov/llama.cpp/discussions/2948)(HuggingFace) models
- [ ] Support a standard web server for inference
- [ ] Support download HF models through [hf-hub](https://github.com/huggingface/hf-hub)
- [ ] Support Nvidia GPUs
- [ ] Support AMD GPUs
- [ ] Support macOS, Linux, Windows, etc.
- [ ] OpenAI compatible [API spec](https://spec.openapis.org/oas/latest.html)
- [ ] Support more GPUs
- [ ] Support LPCP(Large-scale Parallel Central Processing)

## Usage

### Introduction

## License

OpenLLaMA is licensed under the [MIT](https://opensource.org/license/mit).
For detail see [LICENSE](LICENSE).

## Note

The master branch may be in an unstable or even broken state during development. Please use releases instead of the
master branch in order to get a stable set of binaries.

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=computeio/openllama&type=Date)](https://star-history.com/#computeio/openllama&Date)
